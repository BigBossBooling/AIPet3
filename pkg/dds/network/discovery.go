// pkg/dds/network/discovery.go
package network

import (
	"fmt"
)

// PeerDiscovery defines the interface for discovering other DDS peers.
type PeerDiscovery interface {
	DiscoverPeers() ([]Node, error) // Returns a list of discovered DDS nodes
}

// MockPeerDiscovery provides a basic mock implementation for testing.
// It returns a hardcoded list of peers.
type MockPeerDiscovery struct {
	MockPeers     []Node
	SimulateError bool
	ErrorToReturn error
}

// NewMockPeerDiscovery creates a new MockPeerDiscovery instance.
func NewMockPeerDiscovery() *MockPeerDiscovery {
	// Initialize with a few default mock peers for convenience
	peer1, _ := NewNode("localhost:8001", 100)
	peer2, _ := NewNode("localhost:8002", 90)
	peer3, _ := NewNode("192.168.1.100:8000", 110)

	return &MockPeerDiscovery{
		MockPeers: []Node{*peer1, *peer2, *peer3},
	}
}

// DiscoverPeers returns the hardcoded list of mock peers.
func (mpd *MockPeerDiscovery) DiscoverPeers() ([]Node, error) {
	if mpd.SimulateError {
		if mpd.ErrorToReturn != nil {
			return nil, mpd.ErrorToReturn
		}
		return nil, fmt.Errorf("mock peer discovery: simulated error")
	}
	if mpd.MockPeers == nil {
		return []Node{}, nil // Return empty list if not initialized, rather than nil
	}
	// Return a copy to prevent external modification of the internal slice
	peersCopy := make([]Node, len(mpd.MockPeers))
	copy(peersCopy, mpd.MockPeers)
	return peersCopy, nil
}

// AddMockPeer allows tests to add more peers to the discovery list.
func (mpd *MockPeerDiscovery) AddMockPeer(node Node) {
	mpd.MockPeers = append(mpd.MockPeers, node)
}

// ClearMockPeers allows resetting the mock's peer list for tests.
func (mpd *MockPeerDiscovery) ClearMockPeers() {
	mpd.MockPeers = []Node{}
}
