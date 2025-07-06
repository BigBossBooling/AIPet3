// pkg/dds/network/node.go
package network

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
)

// Node represents a peer in the DDS network.
type Node struct {
	ID              string   // Unique identifier for the node (e.g., derived from a public key)
	Address         string   // Network address (e.g., "ip:port" or multiaddr)
	KnownContent    []string // List of CIDs the node currently stores/advertises
	ReputationScore int      // Conceptual link to a reputation system
	// LastSeen      time.Time // Could be added for peer liveness
}

// NewNode creates a new DDS Node.
// For simplicity in this conceptual phase, ID is a random hex string.
// In a real system, ID would likely be cryptographically derived.
func NewNode(address string, reputation int) (*Node, error) {
	if address == "" {
		return nil, fmt.Errorf("node address cannot be empty")
	}

	// Generate a simple pseudo-random ID for mock purposes
	b := make([]byte, 16) // 128-bit random ID
	_, err := rand.Read(b)
	if err != nil {
		return nil, fmt.Errorf("failed to generate random node ID: %w", err)
	}
	id := hex.EncodeToString(b)

	return &Node{
		ID:              id,
		Address:         address,
		KnownContent:    make([]string, 0),
		ReputationScore: reputation,
	}, nil
}

// AddAdvertisedContent adds a CID to the node's list of known content.
func (n *Node) AddAdvertisedContent(cid string) {
	if cid == "" {
		return
	}
	// Avoid duplicates
	for _, existingCID := range n.KnownContent {
		if existingCID == cid {
			return
		}
	}
	n.KnownContent = append(n.KnownContent, cid)
}

// String returns a string representation of the Node.
func (n *Node) String() string {
	return fmt.Sprintf("Node{ID: %s, Address: %s, Reputation: %d, KnownContentCount: %d}",
		n.ID[:8]+"...", // Shorten ID for display
		n.Address,
		n.ReputationScore,
		len(n.KnownContent),
	)
}
