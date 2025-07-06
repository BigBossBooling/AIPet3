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

	fmt.Println("\n--- DDS-Ledger Integration Test Complete ---")
}
