from fastapi.testclient import TestClient

from nexus_anomaly_scorer.main import app


def test_health():
    client = TestClient(app)
    r = client.get("/health")
    assert r.status_code == 200
    assert r.json()["status"] == "ok"


def test_score():
    client = TestClient(app)
    r = client.get("/v1/score")
    assert r.status_code == 200
    body = r.json()
    assert "score" in body
    assert 0 <= body["score"] <= 5
