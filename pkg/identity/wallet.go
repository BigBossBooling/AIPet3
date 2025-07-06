// pkg/identity/wallet.go
package identity

import (
	"crypto/ecdsa"
	"fmt"
)

// Wallet stores key pair and provides wallet functionalities.
type Wallet struct {
	PrivateKey *ecdsa.PrivateKey
	PublicKey  *ecdsa.PublicKey // Derived from PrivateKey
	Address    string           // User-friendly address derived from PublicKey
}

// NewWallet creates a new Wallet with a generated key pair and address.
func NewWallet() (*Wallet, error) {
	privKey, err := GenerateKeyPair()
	if err != nil {
		return nil, fmt.Errorf("failed to generate key pair: %w", err)
	}

	pubKey := &privKey.PublicKey
	address, err := PublicKeyToAddress(pubKey)
	if err != nil {
		return nil, fmt.Errorf("failed to derive address from public key: %w", err)
	}

	return &Wallet{
		PrivateKey: privKey,
		PublicKey:  pubKey,
		Address:    address,
	}, nil
}

// GetAddress returns the wallet's public address.
func (w *Wallet) GetAddress() string {
	return w.Address
}

// GetPublicKeyBytes returns the public key as a byte slice.
func (w *Wallet) GetPublicKeyBytes() ([]byte, error) {
	return PublicKeyToBytes(w.PublicKey)
}

// SignData signs the hash of the given data using the wallet's private key.
func (w *Wallet) SignData(dataHash []byte) ([]byte, error) {
	return Sign(w.PrivateKey, dataHash)
}

// VerifySignature verifies a signature against the wallet's public key.
// pubKeyBytes are the raw public key bytes of the signer.
// This method is more general; for verifying against this wallet's pubkey,
// one would pass w.GetPublicKeyBytes().
func VerifySignatureAgainstKey(pubKeyBytes []byte, dataHash []byte, signature []byte) (bool, error) {
	return VerifySignature(pubKeyBytes, dataHash, signature)
}


// WalletFromPrivateKey reconstructs a Wallet from an existing private key.
func WalletFromPrivateKey(privKey *ecdsa.PrivateKey) (*Wallet, error) {
    if privKey == nil {
        return nil, fmt.Errorf("private key cannot be nil")
    }
    pubKey := &privKey.PublicKey
    address, err := PublicKeyToAddress(pubKey)
    if err != nil {
        return nil, fmt.Errorf("failed to derive address from public key: %w", err)
    }
    return &Wallet{
        PrivateKey: privKey,
        PublicKey:  pubKey,
        Address:    address,
    }, nil
}

// WalletFromPrivateKeyBytes reconstructs a Wallet from private key bytes.
func WalletFromPrivateKeyBytes(privKeyBytes []byte) (*Wallet, error) {
    if len(privKeyBytes) == 0 {
        return nil, fmt.Errorf("private key bytes cannot be empty")
    }
    privKey, err := BytesToPrivateKey(privKeyBytes)
    if err != nil {
        return nil, fmt.Errorf("failed to parse private key from bytes: %w", err)
    }
    return WalletFromPrivateKey(privKey)
}
