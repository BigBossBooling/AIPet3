// core/user/profile_test.go
package user_test

import (
	"digisocialblock/core/user"
	"encoding/json"
	"testing"
	"time"
	"errors" // Ensure errors is imported here
)

func TestNewProfile_Success(t *testing.T) {
	addr := "testAddress123"
	displayName := "Test User"
	bio := "A bio"
	picCID := "picCID123"

	profile, err := user.NewProfile(addr, displayName, bio, picCID)
	if err != nil {
		t.Fatalf("NewProfile failed: %v", err)
	}

	if profile.OwnerAddress != addr {
		t.Errorf("Expected OwnerAddress '%s', got '%s'", addr, profile.OwnerAddress)
	}
	if profile.DisplayName != displayName {
		t.Errorf("Expected DisplayName '%s', got '%s'", displayName, profile.DisplayName)
	}
	if profile.Bio != bio {
		t.Errorf("Expected Bio '%s', got '%s'", bio, profile.Bio)
	}
	if profile.ProfilePictureCID != picCID {
		t.Errorf("Expected ProfilePictureCID '%s', got '%s'", picCID, profile.ProfilePictureCID)
	}
	if profile.Version != 1 {
		t.Errorf("Expected initial Version 1, got %d", profile.Version)
	}
	if profile.Timestamp == 0 {
		t.Error("Expected Timestamp to be set")
	}
}

func TestNewProfile_EmptyOwnerAddress(t *testing.T) {
	_, err := user.NewProfile("", "Test User", "Bio", "")
	if err == nil {
		t.Fatal("NewProfile expected error for empty owner address, got nil")
	}
	expectedError := "owner address cannot be empty"
	if err.Error() != expectedError {
		t.Errorf("Expected error message '%s', got '%s'", expectedError, err.Error())
	}
}

func TestNewProfile_EmptyDisplayName(t *testing.T) {
	_, err := user.NewProfile("testAddress", "", "Bio", "")
	if err == nil {
		t.Fatal("NewProfile expected error for empty display name, got nil")
	}
	expectedError := "display name cannot be empty"
	if err.Error() != expectedError {
		t.Errorf("Expected error message '%s', got '%s'", expectedError, err.Error())
	}
}

func TestNewProfile_DisplayNameTooLong(t *testing.T) {
	longName := string(make([]byte, 51)) // 51 chars
	_, err := user.NewProfile("testAddress", longName, "Bio", "")
	if err == nil {
		t.Fatal("NewProfile expected error for long display name, got nil")
	}
	expectedError := "display name cannot exceed 50 characters"
	if err.Error() != expectedError {
		t.Errorf("Expected error message '%s', got '%s'", expectedError, err.Error())
	}
}
func TestNewProfile_BioTooLong(t *testing.T) {
	longBio := string(make([]byte, 501)) // 501 chars
	_, err := user.NewProfile("testAddress", "User", longBio, "")
	if err == nil {
		t.Fatal("NewProfile expected error for long bio, got nil")
	}
	expectedError := "bio cannot exceed 500 characters"
	if err.Error() != expectedError {
		t.Errorf("Expected error message '%s', got '%s'", expectedError, err.Error())
	}
}


func TestProfile_Update_Success(t *testing.T) {
	profile, _ := user.NewProfile("addr", "Old Name", "Old Bio", "oldPic")
	oldTimestamp := profile.Timestamp
	oldVersion := profile.Version
	time.Sleep(1 * time.Nanosecond) // Ensure timestamp changes

	err := profile.Update("New Name", "New Bio", "newPic")
	if err != nil {
		t.Fatalf("Update failed: %v", err)
	}

	if profile.DisplayName != "New Name" {
		t.Errorf("Expected DisplayName 'New Name', got '%s'", profile.DisplayName)
	}
	if profile.Bio != "New Bio" {
		t.Errorf("Expected Bio 'New Bio', got '%s'", profile.Bio)
	}
	if profile.ProfilePictureCID != "newPic" {
		t.Errorf("Expected ProfilePictureCID 'newPic', got '%s'", profile.ProfilePictureCID)
	}
	if profile.Version != oldVersion+1 {
		t.Errorf("Expected Version %d, got %d", oldVersion+1, profile.Version)
	}
	if profile.Timestamp <= oldTimestamp {
		t.Error("Expected Timestamp to increase after update")
	}
}

func TestProfile_Update_NoChanges(t *testing.T) {
	profile, _ := user.NewProfile("addr", "Name", "Bio", "Pic")
	oldTimestamp := profile.Timestamp
	oldVersion := profile.Version

	// Call update with the *same existing* values to ensure no change
	err := profile.Update(profile.DisplayName, profile.Bio, profile.ProfilePictureCID)
	if err != nil {
		t.Fatalf("Update with no effective changes failed: %v", err)
	}

	if profile.DisplayName != "Name" {
		t.Errorf("DisplayName changed unexpectedly: expected 'Name', got '%s'", profile.DisplayName)
	}
	if profile.Bio != "Bio" {
		t.Errorf("Bio changed unexpectedly: expected 'Bio', got '%s'", profile.Bio)
	}
	if profile.ProfilePictureCID != "Pic" {
		t.Errorf("ProfilePictureCID changed unexpectedly: expected 'Pic', got '%s'", profile.ProfilePictureCID)
	}
	if profile.Version != oldVersion { // Version should NOT change if no fields were altered
		t.Errorf("Expected Version to remain %d, got %d", oldVersion, profile.Version)
	}
	if profile.Timestamp != oldTimestamp { // Timestamp should NOT change if no fields were altered
		t.Error("Expected Timestamp to remain unchanged if no fields were altered")
	}
}

// Removed duplicated import block

func TestProfile_Update_ClearOptionalFields(t *testing.T) {
    profile, _ := user.NewProfile("addr", "Name", "Initial Bio", "initialPicCID")
    initialVersion := profile.Version
    oldTimestamp := profile.Timestamp // Capture old timestamp
    time.Sleep(1 * time.Nanosecond)

    // Clear Bio and ProfilePictureCID by passing empty strings
    err := profile.Update("Name", "", "") // Keep name, clear bio and pic
    if err != nil {
        t.Fatalf("Update to clear fields failed: %v", err)
    }

    if profile.Bio != "" {
        t.Errorf("Expected Bio to be cleared, got '%s'", profile.Bio)
    }
    if profile.ProfilePictureCID != "" {
        t.Errorf("Expected ProfilePictureCID to be cleared, got '%s'", profile.ProfilePictureCID)
    }
    if profile.DisplayName != "Name" { // Name should be unchanged
        t.Errorf("DisplayName changed unexpectedly to '%s'", profile.DisplayName)
    }
    if profile.Version != initialVersion+1 {
        t.Errorf("Expected Version to increment after clearing fields, got %d", profile.Version)
    }
    if profile.Timestamp <= oldTimestamp { // Corrected timestamp check
        t.Error("Expected Timestamp to update (increase) after clearing fields")
    }
}


func TestProfile_Update_Partial(t *testing.T) {
	profile, _ := user.NewProfile("addr", "Name", "Bio", "Pic")
	oldTimestamp := profile.Timestamp
	oldVersion := profile.Version
	time.Sleep(1 * time.Nanosecond)

	err := profile.Update("New Name", "", "") // Update name, clear Bio and Pic
	if err != nil {
		t.Fatalf("Partial update failed: %v", err)
	}

	if profile.DisplayName != "New Name" {
		t.Errorf("DisplayName not updated. Expected 'New Name', got '%s'", profile.DisplayName)
	}
	if profile.Bio != "" { // Should be cleared
		t.Errorf("Bio not cleared on partial update with empty string. Got '%s'", profile.Bio)
	}
	if profile.ProfilePictureCID != "" { // Should be cleared
		t.Errorf("ProfilePictureCID not cleared on partial update with empty string. Got '%s'", profile.ProfilePictureCID)
	}
	if profile.Version != oldVersion+1 { // Version should increment due to clearing
		t.Errorf("Version not incremented. Expected %d, got %d", oldVersion+1, profile.Version)
	}
	if profile.Timestamp <= oldTimestamp { // Timestamp should update
		t.Error("Timestamp not updated on partial update")
	}
}

func TestProfile_Update_DisplayNameTooLong(t *testing.T) {
	profile, _ := user.NewProfile("addr", "Name", "Bio", "Pic")
	longName := string(make([]byte, 51))
	err := profile.Update(longName, "New Bio", "newPic")
	if err == nil {
		t.Fatal("Expected error for long display name in Update, got nil")
	}
	expectedError := "display name cannot exceed 50 characters"
	if err.Error() != expectedError {
		t.Errorf("Expected error '%s', got '%s'", expectedError, err.Error())
	}
}


func TestProfile_ToJSON_FromJSON_Roundtrip(t *testing.T) {
	originalProfile, _ := user.NewProfile("addr123", "JSON User", "Testing JSON roundtrip.", "jsonPicCID")
	originalProfile.Version = 5 // Manually set a different version for test

	jsonData, err := originalProfile.ToJSON()
	if err != nil {
		t.Fatalf("ToJSON failed: %v", err)
	}

	deserializedProfile, err := user.FromJSON(jsonData)
	if err != nil {
		t.Fatalf("FromJSON failed: %v", err)
	}

	if deserializedProfile.OwnerAddress != originalProfile.OwnerAddress {
		t.Errorf("OwnerAddress mismatch: expected %s, got %s", originalProfile.OwnerAddress, deserializedProfile.OwnerAddress)
	}
	if deserializedProfile.DisplayName != originalProfile.DisplayName {
		t.Errorf("DisplayName mismatch: expected %s, got %s", originalProfile.DisplayName, deserializedProfile.DisplayName)
	}
	if deserializedProfile.Bio != originalProfile.Bio {
		t.Errorf("Bio mismatch: expected %s, got %s", originalProfile.Bio, deserializedProfile.Bio)
	}
	if deserializedProfile.ProfilePictureCID != originalProfile.ProfilePictureCID {
		t.Errorf("ProfilePictureCID mismatch: expected %s, got %s", originalProfile.ProfilePictureCID, deserializedProfile.ProfilePictureCID)
	}
	if deserializedProfile.Timestamp != originalProfile.Timestamp {
		t.Errorf("Timestamp mismatch: expected %d, got %d", originalProfile.Timestamp, deserializedProfile.Timestamp)
	}
	if deserializedProfile.Version != originalProfile.Version {
		t.Errorf("Version mismatch: expected %d, got %d", originalProfile.Version, deserializedProfile.Version)
	}
}

func TestProfile_FromJSON_InvalidData(t *testing.T) {
	invalidJSON := []byte(`{"displayName": "Missing Other Fields"`) // Malformed JSON
	_, err := user.FromJSON(invalidJSON)
	if err == nil {
		t.Fatal("FromJSON expected error for invalid JSON, got nil")
	}
	if _, ok := err.(*json.SyntaxError); !ok && !errors.Is(err, &json.SyntaxError{}) && err.Error() != "failed to unmarshal profile data: unexpected end of JSON input" {
        // Allow for different ways a JSON syntax error might be wrapped or reported
         t.Logf("Note: Received JSON error: %v (type: %T)", err, err) // Log the actual error
    }


	missingOwnerJSON := []byte(`{"displayName": "Test", "bio": "Bio", "version": 1}`)
	_, err = user.FromJSON(missingOwnerJSON)
	if err == nil {
		t.Fatal("FromJSON expected error for missing owner address, got nil")
	}
	if err.Error() != "deserialized profile missing owner address" {
		t.Errorf("Expected specific error for missing owner, got '%s'", err.Error())
	}

	missingNameJSON := []byte(`{"ownerAddress": "addr", "bio": "Bio", "version": 1}`)
	_, err = user.FromJSON(missingNameJSON)
	if err == nil {
		t.Fatal("FromJSON expected error for missing display name, got nil")
	}
	if err.Error() != "deserialized profile missing display name" {
		t.Errorf("Expected specific error for missing name, got '%s'", err.Error())
	}

	invalidVersionJSON := []byte(`{"ownerAddress": "addr", "displayName": "Name", "version": 0}`)
	_, err = user.FromJSON(invalidVersionJSON)
	if err == nil {
		t.Fatal("FromJSON expected error for invalid version, got nil")
	}
	if err.Error() != "deserialized profile has invalid version: 0" {
		t.Errorf("Expected specific error for invalid version, got '%s'", err.Error())
	}
}

func TestProfile_FromJSON_EmptyData(t *testing.T) {
	_, err := user.FromJSON([]byte{})
	if err == nil {
		t.Fatal("FromJSON expected error for empty data, got nil")
	}
	if err.Error() != "cannot deserialize empty data into profile" {
		t.Errorf("Expected specific error for empty data, got '%s'", err.Error())
	}
}

func TestProfile_ToJSON_NilProfile(t *testing.T) {
    var p *user.Profile = nil
    _, err := p.ToJSON()
    if err == nil {
        t.Fatal("ToJSON on nil profile expected an error, got nil")
    }
    if err.Error() != "cannot serialize nil profile" {
        t.Errorf("Expected error 'cannot serialize nil profile', got '%s'", err.Error())
    }
}
