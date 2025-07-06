// pkg/dds/network/p2p_service.go
package network

import (
	"digisocialblock/pkg/dds/chunking"
	"fmt"
	"sync"
)

// P2PService defines the interface for peer-to-peer communication
// specific to DDS content exchange.
type P2PService interface {
	// RequestManifest asks a specific peer for a content manifest.
	RequestManifest(peer Node, manifestID string) (*chunking.Manifest, error)

	// RequestChunk asks a specific peer for a specific content chunk.
	RequestChunk(peer Node, chunkID string) (chunking.Chunk, error)

	// AdvertiseContent informs connected peers (or a subset based on strategy)
	// about a new manifestID that this node can provide.
	// In a real system, this might involve broadcasting to a topic or direct messages.
	AdvertiseContent(manifestID string) error

	// Start starts the P2P service (e.g., listening for incoming connections).
	// Start() error // Placeholder for future, not implemented in mock

	// Stop stops the P2P service.
	// Stop() error // Placeholder for future, not implemented in mock
}

// MockP2PService provides a mock implementation for testing P2P interactions.
// It can simulate network responses and keep track of advertisements.
type MockP2PService struct {
	// LocalNode represents the node this service is running on.
	// This helps simulate how it might respond to requests based on its own content.
	LocalNode *Node

	// NetworkView simulates what this node knows about other nodes and their content.
	// Key is peer Node.ID
	NetworkView map[string]*Node // Stores other nodes and their *advertised* content.

	// For direct simulation of request/response without full NetworkView simulation:
	RequestHandlerFunc func(peer Node, requestType string, id string) (interface{}, error)

	AdvertisedToPeers []string // Records manifestIDs advertised by this service

	mu            sync.RWMutex
	SimulateError bool
	ErrorToReturn error
}

// NewMockP2PService creates a new MockP2PService.
func NewMockP2PService(localNode *Node) *MockP2PService {
	if localNode == nil {
		// Create a default local node if none provided for the mock
		localNode, _ = NewNode("localhost:9000", 100) // Default mock local node
	}
	return &MockP2PService{
		LocalNode:   localNode,
		NetworkView: make(map[string]*Node),
		AdvertisedToPeers: make([]string, 0),
	}
}

// RequestManifest simulates requesting a manifest from a peer.
// In this mock, it checks if the target peer (from NetworkView) has the content.
func (mps *MockP2PService) RequestManifest(peer Node, manifestID string) (*chunking.Manifest, error) {
	mps.mu.RLock()
	defer mps.mu.RUnlock()

	if mps.RequestHandlerFunc != nil {
		res, err := mps.RequestHandlerFunc(peer, "manifest", manifestID)
		if err != nil {
			return nil, err
		}
		if manifest, ok := res.(*chunking.Manifest); ok {
			return manifest, nil
		}
		return nil, fmt.Errorf("mock p2p: request handler returned unexpected type for manifest")
	}

	if mps.SimulateError {
		return nil, mps.ErrorToReturn
	}

	targetPeer, exists := mps.NetworkView[peer.ID]
	if !exists {
		return nil, fmt.Errorf("mock p2p: peer %s not found in network view", peer.ID)
	}

	for _, knownCID := range targetPeer.KnownContent {
		if knownCID == manifestID {
			// Peer has it. For mock, we need a way to actually get the manifest.
			// This basic mock assumes if peer "knows" it, it can provide it.
			// A more advanced mock might require seeding the manifest data into the peer's mock storage.
			// For now, let's return a dummy manifest if found.
			return &chunking.Manifest{
				ID:        manifestID,
				ContentID: "mock_content_id_from_" + peer.ID,
				ChunkIDs:  []string{fmt.Sprintf("chunk_for_%s_from_%s", manifestID, peer.ID)},
				TotalSize: 100, // Dummy size
			}, nil
		}
	}
	return nil, fmt.Errorf("mock p2p: peer %s does not advertise manifest %s", peer.ID, manifestID)
}

// RequestChunk simulates requesting a chunk from a peer.
func (mps *MockP2PService) RequestChunk(peer Node, chunkID string) (chunking.Chunk, error) {
	mps.mu.RLock()
	defer mps.mu.RUnlock()

	if mps.RequestHandlerFunc != nil {
		res, err := mps.RequestHandlerFunc(peer, "chunk", chunkID)
		if err != nil {
			return chunking.Chunk{}, err
		}
		if chunk, ok := res.(chunking.Chunk); ok {
			return chunk, nil
		}
		return chunking.Chunk{}, fmt.Errorf("mock p2p: request handler returned unexpected type for chunk")
	}

	if mps.SimulateError {
		return chunking.Chunk{}, mps.ErrorToReturn
	}

	// Simplified: Assume if a peer is in NetworkView, it can provide any chunk asked of it.
	// A real system would check if the peer actually has/advertises the specific chunk
	// or if the manifest implies the peer should have it.
	_, exists := mps.NetworkView[peer.ID]
	if !exists {
		return chunking.Chunk{}, fmt.Errorf("mock p2p: peer %s not found in network view for chunk request", peer.ID)
	}

	// Return a dummy chunk
	return chunking.Chunk{
		ID:   chunkID,
		Data: []byte("mock_chunk_data_from_" + peer.ID),
		Size: len("mock_chunk_data_from_"+peer.ID),
	}, nil
}

// AdvertiseContent simulates the local node advertising content.
// In a real system, this would broadcast to connected peers.
// Here, we just record it.
func (mps *MockP2PService) AdvertiseContent(manifestID string) error {
	mps.mu.Lock()
	defer mps.mu.Unlock()

	if mps.SimulateError && mps.ErrorToReturn != nil {
		return mps.ErrorToReturn
	}

	fmt.Printf("MockP2PService: Node %s advertising manifest %s\n", mps.LocalNode.ID[:8], manifestID)
	mps.LocalNode.AddAdvertisedContent(manifestID) // The local node now knows this content
	mps.AdvertisedToPeers = append(mps.AdvertisedToPeers, manifestID) // Log that an advertisement happened

	// Conceptually, also update other nodes in NetworkView if they were "told"
	// This part is complex for a simple mock. For now, advertising updates the local node's known content.
	// A test could then set up another node's NetworkView to include this local node.
	return nil
}

// Helper methods for testing:

// AddPeerToNetworkView allows tests to populate the simulated network.
func (mps *MockP2PService) AddPeerToNetworkView(peer *Node) {
	mps.mu.Lock()
	defer mps.mu.Unlock()
	if mps.NetworkView == nil {
		mps.NetworkView = make(map[string]*Node)
	}
	mps.NetworkView[peer.ID] = peer
}

// GetLocalNodeAdvertisedContent returns content CIDs advertised by the local node.
func (mps *MockP2PService) GetLocalNodeAdvertisedContent() []string {
	mps.mu.RLock()
	defer mps.mu.RUnlock()
	// Return a copy
	content := make([]string, len(mps.LocalNode.KnownContent))
	copy(content, mps.LocalNode.KnownContent)
	return content
}
