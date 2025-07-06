// pkg/dds/network/p2p_service_test.go
package network_test

import (
	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/network"
	"errors"
	"fmt"
	"testing"
)

func TestMockP2PService_AdvertiseContent_Success(t *testing.T) {
	localNode, _ := network.NewNode("localhost:9000", 100)
	p2pService := network.NewMockP2PService(localNode)
	manifestID := "test_manifest_cid_123"

	err := p2pService.AdvertiseContent(manifestID)
	if err != nil {
		t.Fatalf("AdvertiseContent failed unexpectedly: %v", err)
	}

	advertised := false
	for _, cid := range p2pService.GetLocalNodeAdvertisedContent() {
		if cid == manifestID {
			advertised = true
			break
		}
	}
	if !advertised {
		t.Errorf("Expected manifestID '%s' to be in LocalNode's KnownContent, but it wasn't. Known: %v", manifestID, p2pService.GetLocalNodeAdvertisedContent())
	}

	// Also check AdvertisedToPeers (which logs that an advertisement happened)
	foundInLog := false
	for _, advertisedCID := range p2pService.AdvertisedToPeers {
		if advertisedCID == manifestID {
			foundInLog = true
			break
		}
	}
	if !foundInLog {
		t.Errorf("Expected manifestID '%s' to be logged in AdvertisedToPeers, but it wasn't.", manifestID)
	}
}

func TestMockP2PService_AdvertiseContent_SimulateError(t *testing.T) {
	localNode, _ := network.NewNode("localhost:9001", 100)
	p2pService := network.NewMockP2PService(localNode)
	expectedErr := errors.New("simulated advertise error")
	p2pService.SimulateError = true
	p2pService.ErrorToReturn = expectedErr

	err := p2pService.AdvertiseContent("any_cid")
	if err == nil {
		t.Fatal("AdvertiseContent expected an error, got nil")
	}
	if !errors.Is(err, expectedErr) {
		t.Errorf("Expected error '%v', got '%v'", expectedErr, err)
	}
}

func TestMockP2PService_RequestManifest_Success(t *testing.T) {
	localNode, _ := network.NewNode("requester:9000", 100)
	peerNode, _ := network.NewNode("provider:9001", 100)

	p2pService := network.NewMockP2PService(localNode)

	manifestID := "sample_manifest_id"
	expectedManifest := &chunking.Manifest{
		ID: manifestID, ContentID: "sample_content_id", ChunkIDs: []string{"c1"}, TotalSize: 10,
	}

	// Setup peerNode to "have" the manifest for this test using RequestHandlerFunc
	p2pService.RequestHandlerFunc = func(p network.Node, requestType string, id string) (interface{}, error) {
		if p.ID == peerNode.ID && requestType == "manifest" && id == manifestID {
			return expectedManifest, nil
		}
		return nil, fmt.Errorf("handler: manifest not found or wrong peer/type")
	}

	retrievedManifest, err := p2pService.RequestManifest(*peerNode, manifestID)
	if err != nil {
		t.Fatalf("RequestManifest failed unexpectedly: %v", err)
	}
	if retrievedManifest == nil {
		t.Fatal("RequestManifest returned nil manifest unexpectedly")
	}
	if retrievedManifest.ID != manifestID {
		t.Errorf("RequestManifest returned manifest with wrong ID. Expected %s, got %s", manifestID, retrievedManifest.ID)
	}
}

func TestMockP2PService_RequestManifest_PeerNotFoundInNetworkView_DefaultHandler(t *testing.T) {
    localNode, _ := network.NewNode("requester:9000", 100)
    unknownPeer, _ := network.NewNode("unknown:9002", 100) // This peer is not in NetworkView by default
    p2pService := network.NewMockP2PService(localNode)
    // No RequestHandlerFunc, so it uses default logic which checks NetworkView

    _, err := p2pService.RequestManifest(*unknownPeer, "any_manifest_id")
    if err == nil {
        t.Fatal("RequestManifest expected an error for unknown peer, got nil")
    }
    expectedErrorMsg := fmt.Sprintf("mock p2p: peer %s not found in network view", unknownPeer.ID)
    if err.Error() != expectedErrorMsg {
        t.Errorf("Expected error message '%s', got '%s'", expectedErrorMsg, err.Error())
    }
}


func TestMockP2PService_RequestManifest_KnownPeerNoManifest_DefaultHandler(t *testing.T) {
	localNode, _ := network.NewNode("requester:9000", 100)
	peerNode, _ := network.NewNode("provider:9001", 100)
	peerNode.KnownContent = []string{"other_manifest"} // Peer doesn't have the requested one

	p2pService := network.NewMockP2PService(localNode)
	p2pService.AddPeerToNetworkView(peerNode) // Make peer known

	manifestIDToRequest := "non_existent_manifest_on_peer"
	_, err := p2pService.RequestManifest(*peerNode, manifestIDToRequest)
	if err == nil {
		t.Fatal("RequestManifest expected an error when peer doesn't have manifest, got nil")
	}
	expectedErrorMsg := fmt.Sprintf("mock p2p: peer %s does not advertise manifest %s", peerNode.ID, manifestIDToRequest)
	if err.Error() != expectedErrorMsg {
		t.Errorf("Expected error message '%s', got '%s'", expectedErrorMsg, err.Error())
	}
}


func TestMockP2PService_RequestChunk_Success_WithHandler(t *testing.T) {
	localNode, _ := network.NewNode("requester:9000", 100)
	peerNode, _ := network.NewNode("provider:9001", 100)
	p2pService := network.NewMockP2PService(localNode)

	chunkID := "sample_chunk_id"
	expectedChunk := chunking.Chunk{ID: chunkID, Data: []byte("data"), Size: 4}

	p2pService.RequestHandlerFunc = func(p network.Node, requestType string, id string) (interface{}, error) {
		if p.ID == peerNode.ID && requestType == "chunk" && id == chunkID {
			return expectedChunk, nil
		}
		return nil, fmt.Errorf("handler: chunk not found or wrong peer/type")
	}

	retrievedChunk, err := p2pService.RequestChunk(*peerNode, chunkID)
	if err != nil {
		t.Fatalf("RequestChunk failed unexpectedly: %v", err)
	}
	if retrievedChunk.ID != chunkID {
		t.Errorf("RequestChunk returned chunk with wrong ID. Expected %s, got %s", chunkID, retrievedChunk.ID)
	}
}

func TestMockP2PService_RequestChunk_SimulateError(t *testing.T) {
    localNode, _ := network.NewNode("requester:9000", 100)
    peerNode, _ := network.NewNode("provider:9001", 100)
    p2pService := network.NewMockP2PService(localNode)
    expectedErr := errors.New("simulated chunk request error")
    p2pService.SimulateError = true
    p2pService.ErrorToReturn = expectedErr

    _, err := p2pService.RequestChunk(*peerNode, "any_chunk_id")
    if err == nil {
        t.Fatal("RequestChunk expected an error, got nil")
    }
    if !errors.Is(err, expectedErr) {
        t.Errorf("Expected error '%v', got '%v'", expectedErr, err)
    }
}

func TestMockP2PService_AddPeerToNetworkView(t *testing.T) {
    localNode, _ := network.NewNode("local:9000", 100)
    p2pService := network.NewMockP2PService(localNode)

    peer1, _ := network.NewNode("peer1:8001", 90)
    p2pService.AddPeerToNetworkView(peer1)

    if _, exists := p2pService.NetworkView[peer1.ID]; !exists {
        t.Errorf("Peer1 was not added to NetworkView")
    }

    peer2, _ := network.NewNode("peer2:8002", 80)
    p2pService.AddPeerToNetworkView(peer2)
    if _, exists := p2pService.NetworkView[peer2.ID]; !exists {
        t.Errorf("Peer2 was not added to NetworkView")
    }
    if len(p2pService.NetworkView) != 2 {
        t.Errorf("Expected NetworkView size of 2, got %d", len(p2pService.NetworkView))
    }
}
