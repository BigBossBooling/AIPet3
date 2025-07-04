//! Tests for pallet-critter-minigames

use crate::{mock::*, Error, GameStatus, GameType, GameDifficulty};
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_game_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        
        // Act
        assert_ok!(CritterMinigames::create_game(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameType::LogicLeaper,
            GameDifficulty::Medium
        ));
        
        // Assert
        let game = CritterMinigames::game_instances(0).unwrap();
        assert_eq!(game.pet_id, pet_id);
        assert_eq!(game.owner, account_id);
        assert_eq!(game.game_type, GameType::LogicLeaper);
        assert_eq!(game.difficulty, GameDifficulty::Medium);
        assert_eq!(game.status, GameStatus::InProgress);
        
        let active_games = CritterMinigames::active_games_by_owner(account_id);
        assert_eq!(active_games.len(), 1);
        assert_eq!(active_games[0], 0);
        
        let pet_history = CritterMinigames::pet_game_history(pet_id);
        assert_eq!(pet_history.len(), 1);
        assert_eq!(pet_history[0], 0);
    });
}

#[test]
fn complete_game_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let game_id = 0;
        let score = 500;
        
        assert_ok!(CritterMinigames::create_game(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameType::LogicLeaper,
            GameDifficulty::Medium
        ));
        
        // Act
        assert_ok!(CritterMinigames::complete_game(
            RuntimeOrigin::signed(account_id),
            game_id,
            score
        ));
        
        // Assert
        let game = CritterMinigames::game_instances(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.score, Some(score));
        
        let result = CritterMinigames::game_results(game_id).unwrap();
        assert_eq!(result.score, score);
        
        let active_games = CritterMinigames::active_games_by_owner(account_id);
        assert_eq!(active_games.len(), 0);
    });
}

#[test]
fn abandon_game_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let game_id = 0;
        
        assert_ok!(CritterMinigames::create_game(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameType::LogicLeaper,
            GameDifficulty::Medium
        ));
        
        // Act
        assert_ok!(CritterMinigames::abandon_game(
            RuntimeOrigin::signed(account_id),
            game_id
        ));
        
        // Assert
        let game = CritterMinigames::game_instances(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Abandoned);
        
        let active_games = CritterMinigames::active_games_by_owner(account_id);
        assert_eq!(active_games.len(), 0);
    });
}

#[test]
fn complete_logic_leaper_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let score = 500;
        
        // Act
        assert_ok!(CritterMinigames::complete_logic_leaper(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameDifficulty::Medium,
            score
        ));
        
        // Assert
        let game_id = 0;
        let game = CritterMinigames::game_instances(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.game_type, GameType::LogicLeaper);
        assert_eq!(game.score, Some(score));
        
        let result = CritterMinigames::game_results(game_id).unwrap();
        assert_eq!(result.score, score);
    });
}

#[test]
fn complete_aura_weaving_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let score = 500;
        
        // Act
        assert_ok!(CritterMinigames::complete_aura_weaving(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameDifficulty::Medium,
            score
        ));
        
        // Assert
        let game_id = 0;
        let game = CritterMinigames::game_instances(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.game_type, GameType::AuraWeaving);
        assert_eq!(game.score, Some(score));
        
        let result = CritterMinigames::game_results(game_id).unwrap();
        assert_eq!(result.score, score);
    });
}

#[test]
fn complete_habitat_dash_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let score = 500;
        
        // Act
        assert_ok!(CritterMinigames::complete_habitat_dash(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameDifficulty::Medium,
            score
        ));
        
        // Assert
        let game_id = 0;
        let game = CritterMinigames::game_instances(game_id).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert_eq!(game.game_type, GameType::HabitatDash);
        assert_eq!(game.score, Some(score));
        
        let result = CritterMinigames::game_results(game_id).unwrap();
        assert_eq!(result.score, score);
    });
}

#[test]
fn exceed_max_active_games_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        
        // Create maximum number of games
        for i in 0..MaxActiveGames::get() {
            assert_ok!(CritterMinigames::create_game(
                RuntimeOrigin::signed(account_id),
                pet_id,
                GameType::LogicLeaper,
                GameDifficulty::Medium
            ));
        }
        
        // Act & Assert
        assert_noop!(
            CritterMinigames::create_game(
                RuntimeOrigin::signed(account_id),
                pet_id,
                GameType::LogicLeaper,
                GameDifficulty::Medium
            ),
            Error::<Test>::ExceedMaxActiveGames
        );
    });
}

#[test]
fn not_game_owner_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let other_account_id = 2;
        let pet_id = 0;
        let game_id = 0;
        
        assert_ok!(CritterMinigames::create_game(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameType::LogicLeaper,
            GameDifficulty::Medium
        ));
        
        // Act & Assert
        assert_noop!(
            CritterMinigames::complete_game(
                RuntimeOrigin::signed(other_account_id),
                game_id,
                500
            ),
            Error::<Test>::NotGameOwner
        );
        
        assert_noop!(
            CritterMinigames::abandon_game(
                RuntimeOrigin::signed(other_account_id),
                game_id
            ),
            Error::<Test>::NotGameOwner
        );
    });
}

#[test]
fn game_already_finished_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let game_id = 0;
        
        assert_ok!(CritterMinigames::create_game(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameType::LogicLeaper,
            GameDifficulty::Medium
        ));
        
        assert_ok!(CritterMinigames::complete_game(
            RuntimeOrigin::signed(account_id),
            game_id,
            500
        ));
        
        // Act & Assert
        assert_noop!(
            CritterMinigames::complete_game(
                RuntimeOrigin::signed(account_id),
                game_id,
                600
            ),
            Error::<Test>::GameAlreadyFinished
        );
        
        assert_noop!(
            CritterMinigames::abandon_game(
                RuntimeOrigin::signed(account_id),
                game_id
            ),
            Error::<Test>::GameAlreadyFinished
        );
    });
}

#[test]
fn invalid_score_fails() {
    new_test_ext().execute_with(|| {
        // Arrange
        let account_id = 1;
        let pet_id = 0;
        let game_id = 0;
        let invalid_score = 2000; // Above the maximum allowed score
        
        assert_ok!(CritterMinigames::create_game(
            RuntimeOrigin::signed(account_id),
            pet_id,
            GameType::LogicLeaper,
            GameDifficulty::Medium
        ));
        
        // Act & Assert
        assert_noop!(
            CritterMinigames::complete_game(
                RuntimeOrigin::signed(account_id),
                game_id,
                invalid_score
            ),
            Error::<Test>::InvalidScore
        );
    });
}