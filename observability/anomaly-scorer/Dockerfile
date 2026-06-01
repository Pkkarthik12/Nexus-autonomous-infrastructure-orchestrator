FROM python:3.11-slim
WORKDIR /app
COPY pyproject.toml README.md ./
COPY src ./src
RUN pip install --no-cache-dir .
EXPOSE 8090
CMD ["uvicorn", "nexus_anomaly_scorer.main:app", "--host", "0.0.0.0", "--port", "8090"]
