import chess
import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from safetensors.torch import load_file

from engine.encoding import NUM_PLANES, encode_board

SQUARES = 64
D_MODEL = 320
LAYERS = 10
HEADS = 10
FFN_HIDDEN = 768
HEAD_TOKEN_DIM = 32
HEAD_HIDDEN = 256
BINS = 128
NORM_EPS = 1e-6


class Block(nn.Module):
    def __init__(self):
        super().__init__()
        self.norm1 = nn.RMSNorm(D_MODEL, eps=NORM_EPS)
        self.qkv = nn.Linear(D_MODEL, 3 * D_MODEL, bias=False)
        self.attn_bias = nn.Parameter(torch.zeros(HEADS, SQUARES, SQUARES))
        self.proj = nn.Linear(D_MODEL, D_MODEL, bias=False)
        self.norm2 = nn.RMSNorm(D_MODEL, eps=NORM_EPS)
        self.gate_up = nn.Linear(D_MODEL, 2 * FFN_HIDDEN, bias=False)
        self.down = nn.Linear(FFN_HIDDEN, D_MODEL, bias=False)

    def forward(self, x: torch.Tensor) -> torch.Tensor:
        b = x.shape[0]
        qkv = self.qkv(self.norm1(x)).view(b, SQUARES, 3, HEADS, D_MODEL // HEADS)
        q, k, v = qkv.permute(2, 0, 3, 1, 4)
        a = F.scaled_dot_product_attention(q, k, v, attn_mask=self.attn_bias)
        x = x + self.proj(a.transpose(1, 2).reshape(b, SQUARES, D_MODEL))
        gate, up = self.gate_up(self.norm2(x)).chunk(2, dim=-1)
        return x + self.down(F.silu(gate) * up)


class ValueNet(nn.Module):  # logits from the side to move, tensor names must match the C++ trainer
    def __init__(self):
        super().__init__()
        self.embed = nn.Linear(NUM_PLANES, D_MODEL)
        self.pos = nn.Parameter(torch.zeros(SQUARES, D_MODEL))
        self.blocks = nn.ModuleList(Block() for _ in range(LAYERS))
        self.norm = nn.RMSNorm(D_MODEL, eps=NORM_EPS)
        self.head_token = nn.Linear(D_MODEL, HEAD_TOKEN_DIM)
        self.head_fc = nn.Linear(SQUARES * HEAD_TOKEN_DIM, HEAD_HIDDEN)
        self.head_out = nn.Linear(HEAD_HIDDEN, BINS)

    def forward(self, tokens: torch.Tensor) -> torch.Tensor:
        x = self.embed(tokens) + self.pos
        for block in self.blocks:
            x = block(x)
        x = self.head_token(self.norm(x)).flatten(1)
        return self.head_out(F.silu(self.head_fc(x)))


class Scorer:
    def __init__(self, weights: str | None = None, device: str = "cuda"):
        self.device = torch.device(device)
        self.model = ValueNet()
        if weights is not None:  # None = random weights
            self.model.load_state_dict(load_file(weights), strict=True)
        self.model.to(self.device, torch.bfloat16).eval()
        self.centers = (torch.arange(BINS, device=self.device) + 0.5) / BINS

    @torch.inference_mode()
    def score_batch(self, boards: list[chess.Board]) -> np.ndarray:
        if not boards:
            return np.empty(0, dtype=np.float32)
        planes = torch.from_numpy(np.stack([encode_board(b) for b in boards]))
        tokens = planes.flatten(2).transpose(1, 2).to(self.device, torch.bfloat16)  # (B, 64 squares, 18)
        side_to_move = self.model(tokens).float().softmax(-1) @ self.centers
        black = torch.tensor([b.turn == chess.BLACK for b in boards], device=self.device)
        return torch.where(black, 1 - side_to_move, side_to_move).cpu().numpy()  # 1 = good for White

    def score(self, board: chess.Board) -> float:
        return float(self.score_batch([board])[0])
