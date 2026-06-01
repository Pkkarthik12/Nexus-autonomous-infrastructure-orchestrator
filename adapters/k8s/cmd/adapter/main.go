// K8s execution adapter — Go controller listening on NATS for plans.
package main

import (
	"context"
	"encoding/json"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/nats-io/nats.go"
)

type PlanStep struct {
	Adapter string          `json:"adapter"`
	Action  string          `json:"action"`
	Payload json.RawMessage `json:"payload"`
}

type ExecutionPlan struct {
	PlanID string     `json:"plan_id"`
	Steps  []PlanStep `json:"steps"`
}

func main() {
	natsURL := envOr("NATS_URL", "nats://127.0.0.1:4222")
	subject := envOr("NATS_PLANS_SUBJECT", "nexus.plans.>")

	nc, err := nats.Connect(natsURL)
	if err != nil {
		log.Fatalf("nats connect: %v", err)
	}
	defer nc.Close()

	_, err = nc.Subscribe(subject, func(msg *nats.Msg) {
		var plan ExecutionPlan
		if err := json.Unmarshal(msg.Data, &plan); err != nil {
			log.Printf("invalid plan: %v", err)
			return
		}
		log.Printf("k8s-adapter received plan %s with %d steps", plan.PlanID, len(plan.Steps))
		for _, step := range plan.Steps {
			if step.Adapter != "kubernetes" {
				continue
			}
			applyK8sAction(step.Action, step.Payload)
		}
	})
	if err != nil {
		log.Fatalf("subscribe: %v", err)
	}

	log.Printf("k8s-adapter listening on %s", subject)
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()
	<-ctx.Done()
	log.Println("shutting down")
}

func applyK8sAction(action string, payload json.RawMessage) {
	// Stub: wire client-go InClusterConfig() for production clusters.
	log.Printf("apply action=%s payload=%s", action, string(payload))
	time.Sleep(100 * time.Millisecond)
}

func envOr(key, def string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return def
}
