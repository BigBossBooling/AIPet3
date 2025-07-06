// pkg/dds/network/discovery_test.go
package network_test

import (
	"digisocialblock/pkg/dds/network"
	"errors"
	"testing"
)

func TestMockPeerDiscovery_DiscoverPeers_Default(t *testing.T) {
	mpd := network.NewMockPeerDiscovery()
	peers, err := mpd.DiscoverPeers()

	if err != nil {
		t.Fatalf("DiscoverPeers failed unexpectedly: %v", err)
	}

	// NewMockPeerDiscovery initializes with 3 default peers
	if len(peers) != 3 {
		t.Errorf("Expected 3 default mock peers, got %d", len(peers))
	}

	// Check if one of the default addresses is present (simple check)
	found := false
	for _, p := range peers {
		if p.Address == "localhost:8001" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("Default peer 'localhost:8001' not found in discovered peers")
	}
}

func TestMockPeerDiscovery_DiscoverPeers_Empty(t *testing.T) {
	mpd := network.NewMockPeerDiscovery()
	mpd.ClearMockPeers() // Clear default peers

	peers, err := mpd.DiscoverPeers()
	if err != nil {
		t.Fatalf("DiscoverPeers failed unexpectedly for empty list: %v", err)
	}

	if len(peers) != 0 {
		t.Errorf("Expected 0 peers after ClearMockPeers, got %d", len(peers))
	}
}

func TestMockPeerDiscovery_DiscoverPeers_AddedPeer(t *testing.T) {
	mpd := network.NewMockPeerDiscovery()
	mpd.ClearMockPeers()

	newNode, _ := network.NewNode("test.com:1234", 50)
	mpd.AddMockPeer(*newNode)

	peers, err := mpd.DiscoverPeers()
	if err != nil {
		t.Fatalf("DiscoverPeers failed unexpectedly after adding peer: %v", err)
	}

	if len(peers) != 1 {
		t.Fatalf("Expected 1 added peer, got %d", len(peers))
	}
	if peers[0].ID != newNode.ID || peers[0].Address != "test.com:1234" {
		t.Errorf("Discovered peer does not match added peer. Got %+v, expected %+v", peers[0], *newNode)
	}
}

func TestMockPeerDiscovery_DiscoverPeers_SimulateError(t *testing.T) {
	mpd := network.NewMockPeerDiscovery()
	expectedErr := errors.New("simulated discovery error")
	mpd.SimulateError = true
	mpd.ErrorToReturn = expectedErr

	_, err := mpd.DiscoverPeers()
	if err == nil {
		t.Fatal("DiscoverPeers expected an error, but got nil")
	}

	if !errors.Is(err, expectedErr) {
		t.Errorf("DiscoverPeers error mismatch. Expected '%v', got '%v'", expectedErr, err)
	}
}

func TestMockPeerDiscovery_DiscoverPeers_NilMockPeers(t *testing.T) {
    mpd := network.NewMockPeerDiscovery()
    mpd.MockPeers = nil // Explicitly set to nil to test the nil check

    peers, err := mpd.DiscoverPeers()
    if err != nil {
        t.Fatalf("DiscoverPeers with nil MockPeers failed unexpectedly: %v", err)
    }
    if len(peers) != 0 {
        t.Errorf("Expected 0 peers when MockPeers is nil, got %d", len(peers))
    }
}
