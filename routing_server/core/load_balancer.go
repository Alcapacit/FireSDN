package main

import "fmt"

func balanceTraffic(nodes []string) string {
	if len(nodes) == 0 {
		return "fallback-node"
	}
	// Simple round-robin for demo
	selected := nodes[0]
	fmt.Printf("[Load Balancer] Routing to %s\n", selected)
	return selected
}