// pkg/identity/keys.go
package identity

import (
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/sha256"
	"crypto/x509"
	"encoding/hex"
	"fmt"
	"math/big"
)

// GenerateKeyPair creates a new ECDSA private and public key pair.
func GenerateKeyPair() (*ecdsa.PrivateKey, error) {
	return ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
}

// PublicKeyToAddress converts an ECDSA public key to a hex string address.
// This is a simplified address generation (hash of public key bytes).
func PublicKeyToAddress(pubKey *ecdsa.PublicKey) (string, error) {
	if pubKey == nil || pubKey.X == nil || pubKey.Y == nil {
		return "", fmt.Errorf("public key or its components cannot be nil")
	}
	pubBytes := elliptic.Marshal(elliptic.P256(), pubKey.X, pubKey.Y)
	if pubBytes == nil {
        return "", fmt.Errorf("failed to marshal public key")
    }
	hash := sha256.Sum256(pubBytes)
	return hex.EncodeToString(hash[:]), nil
}

// Sign generates an ECDSA signature for a given data hash.
func Sign(privKey *ecdsa.PrivateKey, dataHash []byte) (signature []byte, err error) {
	if privKey == nil {
		return nil, fmt.Errorf("private key cannot be nil")
	}
	if len(dataHash) == 0 {
        return nil, fmt.Errorf("dataHash cannot be empty")
    }
	r, s, err := ecdsa.Sign(rand.Reader, privKey, dataHash)
	if err != nil {
		return nil, err
	}
	// Signature is R and S concatenated
	// Ensure R and S are fixed size by padding with leading zeros if necessary
    // P256 curve order is 32 bytes for R and S
    rBytes := r.Bytes()
    sBytes := s.Bytes()

    rPadded := make([]byte, 32)
    sPadded := make([]byte, 32)

    copy(rPadded[32-len(rBytes):], rBytes)
    copy(sPadded[32-len(sBytes):], sBytes)

	signature = append(rPadded, sPadded...)
	return signature, nil
}

// VerifySignature verifies an ECDSA signature.
// pubKeyBytes are the raw marshaled public key bytes (SEC1 format).
func VerifySignature(pubKeyBytes []byte, dataHash []byte, signature []byte) (bool, error) {
	if len(pubKeyBytes) == 0 {
        return false, fmt.Errorf("pubKeyBytes cannot be empty")
    }
    if len(dataHash) == 0 {
        return false, fmt.Errorf("dataHash cannot be empty")
    }
    if len(signature) != 64 { // For P256, R and S are 32 bytes each
        return false, fmt.Errorf("signature must be 64 bytes for P256 curve (R + S)")
    }

	curve := elliptic.P256()
	x, y := elliptic.Unmarshal(curve, pubKeyBytes)
	if x == nil { // Unmarshal can return x=nil, y=nil if pubKeyBytes is invalid
		return false, fmt.Errorf("failed to unmarshal public key from bytes")
	}
	publicKey := &ecdsa.PublicKey{Curve: curve, X: x, Y: y}

	r := new(big.Int).SetBytes(signature[:32])
	s := new(big.Int).SetBytes(signature[32:])

	return ecdsa.Verify(publicKey, dataHash, r, s), nil
}

// PrivateKeyToBytes converts an ecdsa.PrivateKey to a byte slice (DER format).
func PrivateKeyToBytes(privKey *ecdsa.PrivateKey) ([]byte, error) {
	if privKey == nil {
		return nil, fmt.Errorf("private key cannot be nil")
	}
	return x509.MarshalECPrivateKey(privKey)
}

// BytesToPrivateKey converts a byte slice (DER format) back to an ecdsa.PrivateKey.
func BytesToPrivateKey(privKeyBytes []byte) (*ecdsa.PrivateKey, error) {
	if len(privKeyBytes) == 0 {
        return nil, fmt.Errorf("private key bytes cannot be empty")
    }
	return x509.ParseECPrivateKey(privKeyBytes)
}

// PublicKeyToBytes converts an ecdsa.PublicKey to a byte slice (marshaled format).
func PublicKeyToBytes(pubKey *ecdsa.PublicKey) ([]byte, error) {
	if pubKey == nil || pubKey.X == nil || pubKey.Y == nil {
		return nil, fmt.Errorf("public key or its components cannot be nil")
	}
	marshaledKey := elliptic.Marshal(elliptic.P256(), pubKey.X, pubKey.Y)
	if marshaledKey == nil {
        return nil, fmt.Errorf("failed to marshal public key")
    }
    return marshaledKey, nil
}

// BytesToPublicKey converts a byte slice (marshaled format) back to an ecdsa.PublicKey.
func BytesToPublicKey(pubKeyBytes []byte) (*ecdsa.PublicKey, error) {
	if len(pubKeyBytes) == 0 {
        return nil, fmt.Errorf("public key bytes cannot be empty")
    }
	curve := elliptic.P256()
	x, y := elliptic.Unmarshal(curve, pubKeyBytes)
	if x == nil { // Unmarshal can return x=nil, y=nil if pubKeyBytes is invalid
		return nil, fmt.Errorf("failed to unmarshal public key from bytes")
	}
	return &ecdsa.PublicKey{Curve: curve, X: x, Y: y}, nil
}
