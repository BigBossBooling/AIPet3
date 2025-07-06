// pkg/ledger/transaction.go
package ledger

import (
	"crypto/sha256"
	"digisocialblock/pkg/identity" // Correctly import the identity package
	"encoding/hex"
	"fmt"
	"time"
)

// Sign populates the SenderPublicKeyBytes and Signature fields of the transaction.
// It requires the sender's private key.
func (tx *Transaction) Sign(privKeyBytes []byte) error {
	if tx == nil {
		return fmt.Errorf("transaction cannot be nil")
	}

	privKey, err := identity.BytesToPrivateKey(privKeyBytes)
	if err != nil {
		return fmt.Errorf("failed to reconstruct private key for signing: %w", err)
	}

	pubKeyBytes, err := identity.PublicKeyToBytes(&privKey.PublicKey)
	if err != nil {
		return fmt.Errorf("failed to get public key bytes: %w", err)
	}
	tx.SenderPublicKey = pubKeyBytes // Store public key bytes

	hash, err := tx.CalculateHash()
	if err != nil {
		return fmt.Errorf("failed to calculate hash for signing: %w", err)
	}

	sig, err := identity.Sign(privKey, hash)
	if err != nil {
		return fmt.Errorf("failed to sign transaction: %w", err)
	}
	tx.Signature = sig
	return nil
}

// VerifySignature checks if the transaction's signature is valid
// using the SenderPublicKey stored in the transaction.
func (tx *Transaction) VerifySignature() bool {
	if tx == nil || len(tx.SenderPublicKey) == 0 || len(tx.Signature) == 0 {
		// fmt.Println("VerifySignature: Nil transaction, or missing public key or signature")
		return false
	}

	hash, err := tx.CalculateHash()
	if err != nil {
		// fmt.Printf("VerifySignature: Error calculating hash: %v\n", err)
		return false
	}

	verified, err := identity.VerifySignature(tx.SenderPublicKey, hash, tx.Signature)
	if err != nil {
		// fmt.Printf("VerifySignature: Error during cryptographic verification: %v\n", err)
		return false
	}
	return verified
}


// Note: The CalculateHash, NewTransaction, Transaction struct, TransactionType const, etc.,
// remain in model.go as they define the core structure and creation.
// This file transaction.go is specifically for the signing and verification logic
// that directly uses the identity package.

// Re-define CalculateHash here if it's not accessible or to avoid import cycle,
// but it's better if it's a method on Transaction in model.go
// For now, let's assume CalculateHash is defined on *Transaction in model.go
// and tx.CalculateHash() is callable.

// If we need to re-define NewTransaction or other parts here due to refactoring,
// ensure that model.go is updated accordingly to avoid duplication.
// For this task, we are focusing on moving Sign and VerifySignature.

// Helper to ensure tx.ID is set if not already.
// This is typically done in NewTransaction.
func (tx *Transaction) ensureID() {
    if tx.ID == "" {
        // Simplified ID generation for this context if NewTransaction wasn't used.
        // A robust system would always use NewTransaction.
        idDataStr := time.Unix(0, tx.Timestamp).String() + string(tx.Type) + tx.SenderAddress + hex.EncodeToString(tx.Payload)
        idHash := sha256.Sum256([]byte(idDataStr))
        tx.ID = hex.EncodeToString(idHash[:])
    }
}
