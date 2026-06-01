package main

import "testing"

func TestEnvOr(t *testing.T) {
	if got := envOr("UNSET_VAR_XYZ", "default"); got != "default" {
		t.Fatalf("expected default, got %s", got)
	}
}
