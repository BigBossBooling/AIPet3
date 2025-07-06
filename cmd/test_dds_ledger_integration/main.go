// cmd/test_dds_ledger_integration/main.go
package main

import (
	"fmt"
	"log"
	"digisocialblock/pkg/ledger"
	"digisocialblock/pkg/identity"
	"digisocialblock/core/content"
	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/storage"
	"digisocialblock/pkg/dds/originator"
	"digisocialblock/pkg/dds/retriever" // Added for ContentRetriever
)

func main() {
	fmt.Println("--- Digisocialblock DDS-Ledger Integration Test ---")

	// 1. Initialize Blockchain
	blockchain := ledger.NewBlockchain()

	// 2. Generate a Wallet for a mock user
	userWallet, err := identity.NewWallet()
	if err != nil {
		log.Fatalf("Failed to create user wallet: %v", err)
	}
	fmt.Printf("\nUser Wallet created: %s\n", userWallet.GetAddress())

	// 3. Initialize DDS Mocks for ContentPublisher
	basicChunker := chunking.NewBasicChunker(10) // Chunk size 10 bytes for testing
	inMemoryStorage := storage.NewInMemoryStorage()
	mockOriginator := originator.NewMockOriginator()
	contentPublisher := content.NewContentPublisher(basicChunker, inMemoryStorage, mockOriginator)

	// 4. Sample Post Content
	samplePostContent := "This is my first decentralized post on Digisocialblock! It's going to be awesome."
	fmt.Printf("\nSample post content: \"%s\"\n", samplePostContent)

	// 5. Publish content to DDS
	contentCID, err := contentPublisher.PublishContent([]byte(samplePostContent))
	if err != nil {
		log.Fatalf("Failed to publish content to DDS: %v", err)
	}
	fmt.Printf("Content published to DDS. Manifest CID: %s\n", contentCID)

	// 6. Create a PostCreated Transaction
	txPayload := []byte(contentCID) // Payload is the CID
	postTx, err := ledger.NewTransaction(userWallet.GetAddress(), ledger.TxTypePostCreated, txPayload)
	if err != nil {
		log.Fatalf("Failed to create post transaction: %v", err)
	}

	// 7. Sign the Transaction
	privateKeyBytes, err := identity.PrivateKeyToBytes(userWallet.PrivateKey)
	if err != nil {
		log.Fatalf("Failed to get private key bytes: %v", err)
	}
	if err := postTx.Sign(privateKeyBytes); err != nil {
		log.Fatalf("Failed to sign transaction: %v", err)
	}
	fmt.Printf("Transaction created and signed (ID: %s)\n", postTx.ID)

	// 8. Verify the Transaction Signature (self-check before adding to block)
	if !postTx.VerifySignature() {
		log.Fatalf("Transaction signature verification failed unexpectedly!")
	}
	fmt.Println("Transaction signature verified successfully.")

	// 9. Add the Transaction to the Blockchain (in a new block)
	_, err = blockchain.AddBlock([]ledger.Transaction{*postTx}) // Dereference postTx
	if err != nil {
		log.Fatalf("Failed to add block to blockchain: %v", err)
	}

	// 10. Verify the Blockchain's integrity
	if !blockchain.IsChainValid() {
		log.Fatalf("Blockchain is invalid after adding block!")
	}
	fmt.Println("Blockchain is valid.")

	// 11. Retrieve the transaction from the blockchain and inspect its payload
	latestBlock := blockchain.GetLatestBlock()
	if len(latestBlock.Transactions) > 0 {
		retrievedTx := latestBlock.Transactions[0]
		retrievedCID := string(retrievedTx.Payload)
		fmt.Printf("\nRetrieved transaction from blockchain:\n")
		fmt.Printf("  Type: %s\n", retrievedTx.Type)
		fmt.Printf("  Sender: %s\n", retrievedTx.SenderPublicKey)
		fmt.Printf("  Payload (CID): %s\n", retrievedCID)

		if retrievedCID == contentCID {
			fmt.Println("  Payload CID matches original content CID. Link established!")
		} else {
			fmt.Println("  ERROR: Payload CID does NOT match original content CID.")
		}
	}

	// --- Steps for Content Retrieval ---
	fmt.Println("\n--- Testing Content Retrieval from DDS via Ledger CID ---")

	// 12. Initialize MockRetriever and ContentRetriever
	// We need to make the InMemoryStorage accessible to seed the MockRetriever,
	// or the MockRetriever needs a way to access the data stored by the publisher.
	// For this test, let's assume InMemoryStorage can be accessed or we manually populate MockRetriever.

	// Get the stored manifest and chunks from inMemoryStorage (used by publisher)
	// This simulates the retriever having access to the same underlying storage for this test.
	manifest, err := inMemoryStorage.GetManifest(contentCID)
	if err != nil {
		log.Fatalf("Failed to get manifest %s from inMemoryStorage for retriever setup: %v", contentCID, err)
	}

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest) // Seed manifest

	for _, chunkID := range manifest.ChunkIDs {
		chunk, err := inMemoryStorage.GetChunk(chunkID)
		if err != nil {
			log.Fatalf("Failed to get chunk %s from inMemoryStorage for retriever setup: %v", chunkID, err)
		}
		mockRetriever.AddChunk(chunk) // Seed chunks
	}

	contentRetriever := content.NewContentRetriever(mockRetriever)

	// 13. Retrieve content using the CID from the blockchain transaction
	if len(latestBlock.Transactions) > 0 {
		retrievedTx := latestBlock.Transactions[0]
		cidFromLedger := string(retrievedTx.Payload)

		fmt.Printf("Attempting to retrieve content from DDS using CID from ledger: %s\n", cidFromLedger)
		retrievedPostContentBytes, err := contentRetriever.RetrieveContent(cidFromLedger)
		if err != nil {
			log.Fatalf("Failed to retrieve content from DDS: %v", err)
		}
		retrievedPostContent := string(retrievedPostContentBytes)
		fmt.Printf("Content retrieved from DDS: \"%s\"\n", retrievedPostContent)

		// 14. Verify retrieved content matches original
		if retrievedPostContent == samplePostContent {
			fmt.Println("SUCCESS: Retrieved content matches original sample post content!")
		} else {
			fmt.Println("ERROR: Retrieved content DOES NOT MATCH original sample post content.")
			log.Fatalf("Mismatch: \nOriginal: %s\nRetrieved: %s", samplePostContent, retrievedPostContent)
		}
	} else {
		log.Println("No transactions found in the latest block to test retrieval.")
	}

	fmt.Println("\n--- DDS-Ledger Integration Test Complete (including retrieval) ---")
}
