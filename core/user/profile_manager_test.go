// core/user/profile_manager_test.go
package user_test

import (
	"crypto/sha256"
	"digisocialblock/core/content"
	"digisocialblock/core/user"
	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/originator"
	"digisocialblock/pkg/dds/retriever"
	"digisocialblock/pkg/dds/storage"
	"encoding/hex"
	"errors"
	"fmt"
	"strings"
	"testing"
)

// Helper to create a consistent hash for content
func hashTestData(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}

// Helper function to setup ProfileManager with actual content services using DDS mocks
func setupProfileManagerWithRealContentServicesUsingMocks(t *testing.T) (
	pm *user.ProfileManager,
	mockChunker *chunking.MockChunker,
	mockStorage *storage.MockStorage,
	mockOriginator *originator.MockOriginator,
	mockDdsRetriever *retriever.MockRetriever,
) {
	mockChunker = chunking.NewMockChunker()       // Uses hashData for IDs by default now
	mockStorage = storage.NewMockStorage()       // Default funcs now use internal maps correctly
	mockOriginator = originator.NewMockOriginator()
	mockDdsRetriever = retriever.NewMockRetriever() // This is for ContentRetriever's dependency

	actualContentPublisher := content.NewContentPublisher(mockChunker, mockStorage, mockOriginator)
	actualContentRetriever := content.NewContentRetriever(mockDdsRetriever) // ContentRetriever uses a retriever.Retriever

	profileManager := user.NewProfileManager(actualContentPublisher, actualContentRetriever)

	return profileManager, mockChunker, mockStorage, mockOriginator, mockDdsRetriever
}

func TestProfileManager_PublishProfile_Success(t *testing.T) {
	profileManager, _, mockStorage, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)

	testProfile, _ := user.NewProfile("addr1", "Test User", "Bio", "pic1")

	cid, err := profileManager.PublishProfile(testProfile)
	if err != nil {
		t.Fatalf("PublishProfile failed unexpectedly: %v", err)
	}

	if cid == "" {
		t.Error("Expected a non-empty CID, got empty string")
	}

	publishedManifest, manErr := mockStorage.GetManifest(cid)
	if manErr != nil {
		t.Fatalf("Failed to get published manifest from mock storage (CID: %s): %v", cid, manErr)
	}
	if publishedManifest.ID != cid {
		t.Errorf("Stored manifest ID mismatch. Expected %s, got %s", cid, publishedManifest.ID)
	}

	tempMockDdsRetriever := retriever.NewMockRetriever()
	tempMockDdsRetriever.FetchManifestFunc = func(manifestCID string) (*chunking.Manifest, error) {
		return mockStorage.GetManifest(manifestCID)
	}
	tempMockDdsRetriever.FetchChunkFunc = func(chunkCID string) (chunking.Chunk, error) {
		return mockStorage.GetChunk(chunkCID)
	}
	tempContentRetriever := content.NewContentRetriever(tempMockDdsRetriever)

	retrievedData, retrErr := tempContentRetriever.RetrieveContent(cid)
	if retrErr != nil {
		t.Fatalf("Failed to retrieve content for verification using temp retriever: %v", retrErr)
	}
	profileFromStorage, jsonErr := user.FromJSON(retrievedData)
	if jsonErr != nil {
		t.Fatalf("Failed to deserialize data from mock storage via temp retriever: %v", jsonErr)
	}
	if profileFromStorage.DisplayName != testProfile.DisplayName {
		t.Errorf("DisplayName mismatch. Expected '%s', got '%s'", testProfile.DisplayName, profileFromStorage.DisplayName)
	}
}

func TestProfileManager_PublishProfile_NilProfile(t *testing.T) {
	profileManager, _, _, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)
	_, err := profileManager.PublishProfile(nil)
	if err == nil {
		t.Fatal("PublishProfile expected error for nil profile, got nil")
	}
	if err.Error() != "cannot publish a nil profile" {
		t.Errorf("Expected error 'cannot publish a nil profile', got '%s'", err.Error())
	}
}

func TestProfileManager_PublishProfile_PublisherError(t *testing.T) {
	profileManager, mockChunker, _, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)
	mockChunker.ChunkContentFunc = func(content []byte) ([]chunking.Chunk, error) {
		return nil, errors.New("simulated chunker error in publisher")
	}
	testProfile, _ := user.NewProfile("addr2", "User", "Bio", "")
	_, err := profileManager.PublishProfile(testProfile)
	if err == nil {
		t.Fatal("PublishProfile expected error from publisher (via chunker), got nil")
	}
	if !strings.Contains(err.Error(), "failed to publish profile data to DDS") || !strings.Contains(err.Error(), "failed to chunk content: simulated chunker error in publisher") {
		t.Errorf("Unexpected error message: %s", err.Error())
	}
}

func TestProfileManager_RetrieveProfile_Success(t *testing.T) {
	profileManager, _, _, _, mockDdsRetriever := setupProfileManagerWithRealContentServicesUsingMocks(t)
	testProfile, _ := user.NewProfile("addr3", "Retrieve User", "Bio for retrieval", "picRet")
	profileData, _ := testProfile.ToJSON()
	testManifestCID := "manifest_cid_for_retrieve_success"

	actualDataChunkID := hashTestData(profileData)
	mockManifest := &chunking.Manifest{
		ID:        testManifestCID,
		ContentID: hashTestData(profileData),
		ChunkIDs:  []string{actualDataChunkID},
		TotalSize: int64(len(profileData)),
	}
	mockChunk := chunking.Chunk{
		ID:   actualDataChunkID,
		Data: profileData,
		Size: len(profileData),
	}
	mockDdsRetriever.FetchManifestFunc = func(manifestCID string) (*chunking.Manifest, error) {
		if manifestCID == testManifestCID {
			return mockManifest, nil
		}
		return nil, fmt.Errorf("manifest %s not found in mockDdsRetriever", manifestCID)
	}
	mockDdsRetriever.FetchChunkFunc = func(chunkCID string) (chunking.Chunk, error) {
		if chunkCID == actualDataChunkID {
			return mockChunk, nil
		}
		return chunking.Chunk{}, fmt.Errorf("chunk %s not found in mockDdsRetriever", chunkCID)
	}
	retrievedProfile, err := profileManager.RetrieveProfile(testManifestCID)
	if err != nil {
		t.Fatalf("RetrieveProfile failed unexpectedly: %v", err)
	}
	if retrievedProfile.DisplayName != testProfile.DisplayName {
		t.Errorf("Retrieved profile DisplayName mismatch. Expected '%s', got '%s'", testProfile.DisplayName, retrievedProfile.DisplayName)
	}
	if retrievedProfile.Bio != testProfile.Bio {
		t.Errorf("Retrieved profile Bio mismatch. Expected '%s', got '%s'", testProfile.Bio, retrievedProfile.Bio)
	}
}

func TestProfileManager_RetrieveProfile_EmptyCID(t *testing.T) {
	profileManager, _, _, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)
	_, err := profileManager.RetrieveProfile("")
	if err == nil {
		t.Fatal("RetrieveProfile expected error for empty CID, got nil")
	}
	if err.Error() != "profile CID cannot be empty for retrieval" {
		t.Errorf("Expected error 'profile CID cannot be empty for retrieval', got '%s'", err.Error())
	}
}

func TestProfileManager_RetrieveProfile_RetrieverError(t *testing.T) {
	profileManager, _, _, _, mockDdsRetriever := setupProfileManagerWithRealContentServicesUsingMocks(t)
	mockDdsRetriever.FetchManifestFunc = func(contentCID string) (*chunking.Manifest, error) {
		return nil, retriever.ErrSimulatedRetriever // Use predefined error
	}
	testCID := "cid_retriever_fails"
	_, err := profileManager.RetrieveProfile(testCID)
	if err == nil {
		t.Fatal("RetrieveProfile expected error from retriever, got nil")
	}
	if !errors.Is(err, retriever.ErrSimulatedRetriever) {
		t.Errorf("Expected error chain to include '%v', got '%v'", retriever.ErrSimulatedRetriever, err)
	}
	if !strings.Contains(err.Error(), "failed to retrieve profile data") {
        t.Errorf("Error message should indicate failure to retrieve profile data, got: %v", err)
    }
}

func TestProfileManager_RetrieveProfile_DeserializationError(t *testing.T) {
	profileManager, _, _, _, mockDdsRetriever := setupProfileManagerWithRealContentServicesUsingMocks(t)
	testCID := "cid_bad_json"
	badJsonData := []byte("this is not valid json")
	badDataChunkID := hashTestData(badJsonData)
	badDataManifest := &chunking.Manifest{
		ID:        testCID,
		ContentID: hashTestData(badJsonData),
		ChunkIDs:  []string{badDataChunkID},
		TotalSize: int64(len(badJsonData)),
	}
	badDataChunk := chunking.Chunk{
		ID:   badDataChunkID,
		Data: badJsonData,
		Size: len(badJsonData),
	}
	mockDdsRetriever.FetchManifestFunc = func(mcid string) (*chunking.Manifest, error) {
		if mcid == testCID {
			return badDataManifest, nil
		}
		return nil, fmt.Errorf("manifest %s not found for bad JSON test", mcid)
	}
	mockDdsRetriever.FetchChunkFunc = func(ccid string) (chunking.Chunk, error) {
		if ccid == badDataChunkID {
			return badDataChunk, nil
		}
		return chunking.Chunk{}, fmt.Errorf("chunk %s not found for bad JSON test", ccid)
	}
	_, err := profileManager.RetrieveProfile(testCID)
	if err == nil {
		t.Fatal("RetrieveProfile expected error from JSON deserialization, got nil")
	}
	if !strings.Contains(err.Error(), "failed to deserialize profile data") {
		t.Errorf("Expected error to mention deserialization failure, got: %s", err.Error())
	}
}

func TestProfileManager_UpdateAndPublishProfile_Success(t *testing.T) {
	profileManager, _, mockStorage, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)
	initialProfile, _ := user.NewProfile("addr4", "Original Name", "Original Bio", "")
	initialCID, err := profileManager.PublishProfile(initialProfile)
	if err != nil {
		t.Fatalf("Initial publish failed: %v", err)
	}
	if initialProfile.Version != 1 {
		t.Fatalf("Initial profile version should be 1, got %d", initialProfile.Version)
	}
	updatedDisplayName := "Updated Name"
	updatedBio := "Updated Bio here."
	updatedPicCID := "newPicCID123"
	newCID, updatedProfile, err := profileManager.UpdateAndPublishProfile(initialProfile, updatedDisplayName, updatedBio, updatedPicCID)
	if err != nil {
		t.Fatalf("UpdateAndPublishProfile failed: %v", err)
	}
	if newCID == "" || newCID == initialCID {
		t.Errorf("Expected a new, non-empty CID after update and publish, got '%s'", newCID)
	}
	if updatedProfile.DisplayName != updatedDisplayName {
		t.Errorf("Updated profile DisplayName mismatch. Expected '%s', got '%s'", updatedDisplayName, updatedProfile.DisplayName)
	}
	if updatedProfile.Version != 2 {
		t.Errorf("Expected profile version to be 2 after update, got %d", updatedProfile.Version)
	}
	_, manErr := mockStorage.GetManifest(newCID)
	if manErr != nil {
		t.Fatalf("Failed to get updated manifest from mock storage (CID: %s): %v", newCID, manErr)
	}
}

func TestProfileManager_UpdateAndPublishProfile_NilProfile(t *testing.T) {
	profileManager, _, _, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)
	_, _, err := profileManager.UpdateAndPublishProfile(nil, "New Name", "New Bio", "newPic")
	if err == nil {
		t.Fatal("UpdateAndPublishProfile expected error for nil profile, got nil")
	}
	if err.Error() != "current profile cannot be nil for update" {
		t.Errorf("Expected error 'current profile cannot be nil for update', got '%s'", err.Error())
	}
}

func TestProfileManager_UpdateAndPublishProfile_UpdateFails(t *testing.T) {
	profileManager, _, _, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)
	testProfile, _ := user.NewProfile("addr5", "Valid Name", "Valid Bio", "")
	longDisplayName := string(make([]byte, 100))
	_, _, err := profileManager.UpdateAndPublishProfile(testProfile, longDisplayName, "Bio", "pic")
	if err == nil {
		t.Fatal("UpdateAndPublishProfile expected error due to profile update validation, got nil")
	}
	if !strings.Contains(err.Error(), "failed to update profile struct: display name cannot exceed 50 characters") {
		t.Errorf("Unexpected error message for update failure: %s", err.Error())
	}
}

func TestProfileManager_UpdateAndPublishProfile_PublishFails(t *testing.T) {
	profileManager, mockChunker, _, _, _ := setupProfileManagerWithRealContentServicesUsingMocks(t)
	mockChunker.ChunkContentFunc = func(content []byte) ([]chunking.Chunk, error) {
		return nil, errors.New("simulated chunker error during update-publish")
	}
	testProfile, _ := user.NewProfile("addr6", "Name", "Bio", "")
	originalVersion := testProfile.Version
	originalTimestamp := testProfile.Timestamp
	_, updatedProfileAfterFail, err := profileManager.UpdateAndPublishProfile(testProfile, "NewName", "NewBio", "NewPic")
	if err == nil {
		t.Fatal("UpdateAndPublishProfile expected error due to publisher failing, got nil")
	}
	if !strings.Contains(err.Error(), "profile struct updated, but failed to publish updated profile") ||
		!strings.Contains(err.Error(), "failed to chunk content: simulated chunker error during update-publish") {
		t.Errorf("Unexpected error message for publish failure during update: %s", err.Error())
	}
	if updatedProfileAfterFail.DisplayName != "NewName" {
		t.Error("Profile DisplayName should have been updated in memory even if publish failed.")
	}
	if updatedProfileAfterFail.Version == originalVersion {
		t.Error("Profile Version should have been incremented in memory even if publish failed.")
	}
	if updatedProfileAfterFail.Timestamp == originalTimestamp {
		t.Error("Profile Timestamp should have been updated in memory even if publish failed.")
	}
}
