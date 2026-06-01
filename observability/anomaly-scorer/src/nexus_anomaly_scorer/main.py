"""FastAPI anomaly scoring service."""

from __future__ import annotations

import os
import random
from contextlib import asynccontextmanager

import numpy as np
from fastapi import FastAPI
from sklearn.ensemble import IsolationForest

_model: IsolationForest | None = None


def _train_stub_model() -> IsolationForest:
    rng = np.random.default_rng(42)
    normal = rng.normal(0, 1, (500, 3))
    model = IsolationForest(contamination=0.05, random_state=42)
    model.fit(normal)
    return model


@asynccontextmanager
async def lifespan(_app: FastAPI):
    global _model
    _model = _train_stub_model()
    yield


app = FastAPI(title="Nexus Anomaly Scorer", version="0.1.0", lifespan=lifespan)


@app.get("/health")
def health():
    return {"status": "ok"}


@app.get("/v1/score")
def score():
    """Return anomaly score (higher = more anomalous). Sentinel uses z-score threshold."""
    assert _model is not None
    sample = np.array([[random.gauss(0, 1), random.gauss(0, 1), random.gauss(0, 1)]])
    raw = -_model.decision_function(sample)[0]
    # Scale to roughly 0–5 for demo
    scaled = float(max(0.0, min(5.0, (raw + 0.5) * 4)))
    return {"score": scaled, "model": "isolation_forest"}


if __name__ == "__main__":
    import uvicorn

    port = int(os.environ.get("PORT", "8090"))
    uvicorn.run("nexus_anomaly_scorer.main:app", host="0.0.0.0", port=port, reload=False)
