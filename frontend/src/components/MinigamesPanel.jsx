import React, { useState, useEffect } from 'react';
import { Card, Button, Select, Table, Tag, Modal, InputNumber, Spin, notification, Tabs, Statistic, Row, Col } from 'antd';
import { 
  TrophyOutlined, 
  RocketOutlined, 
  BulbOutlined, 
  ClockCircleOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  PlayCircleOutlined
} from '@ant-design/icons';
import critterCraftAPI from '../crittercraft_api';

const { Option } = Select;
const { TabPane } = Tabs;

/**
 * MinigamesPanel component for starting and managing mini-games
 */
const MinigamesPanel = ({ pets = [] }) => {
  const [loading, setLoading] = useState(true);
  const [activeGames, setActiveGames] = useState([]);
  const [completedGames, setCompletedGames] = useState([]);
  const [startGameModalVisible, setStartGameModalVisible] = useState(false);
  const [submitScoreModalVisible, setSubmitScoreModalVisible] = useState(false);
  const [selectedPet, setSelectedPet] = useState(null);
  const [selectedGameType, setSelectedGameType] = useState('LogicLeaper');
  const [selectedDifficulty, setSelectedDifficulty] = useState('Easy');
  const [selectedGameId, setSelectedGameId] = useState(null);
  const [score, setScore] = useState(1000);
  const [actionLoading, setActionLoading] = useState(false);
  const [refreshing, setRefreshing] = useState(false);

  // Game type options
  const gameTypes = [
    { value: 'LogicLeaper', label: 'Logic Leaper', description: 'A puzzle game that trains Intelligence' },
    { value: 'AuraWeaving', label: 'Aura Weaving', description: 'A rhythm game that trains Charisma' },
    { value: 'HabitatDash', label: 'Habitat Dash', description: 'An endless runner that trains Agility' },
  ];

  // Difficulty options
  const difficultyLevels = [
    { value: 'Easy', label: 'Easy', multiplier: '1x' },
    { value: 'Medium', label: 'Medium', multiplier: '2x' },
    { value: 'Hard', label: 'Hard', multiplier: '3x' },
    { value: 'Expert', label: 'Expert', multiplier: '4x' },
  ];

  // Fetch active games on component mount
  useEffect(() => {
    fetchGames();
  }, []);

  // Fetch active and completed games
  const fetchGames = async () => {
    try {
      setRefreshing(true);
      const activeGameIds = await critterCraftAPI.getActiveGamesByPlayer();
      
      // Fetch details for each active game
      const activeGamesPromises = activeGameIds.map(id => critterCraftAPI.getGame(id));
      const activeGamesData = await Promise.all(activeGamesPromises);
      
      // Fetch completed games (this is a placeholder - in a real app, you'd have an API for this)
      // For now, we'll just use a mock
      const completedGamesData = []; // Mock data would go here
      
      setActiveGames(activeGamesData);
      setCompletedGames(completedGamesData);
    } catch (error) {
      console.error('Failed to fetch games:', error);
      notification.error({
        message: 'Failed to fetch games',
        description: error.message,
      });
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  };

  // Start a new game
  const handleStartGame = async () => {
    if (!selectedPet) {
      notification.warning({
        message: 'No pet selected',
        description: 'Please select a pet to play the game.',
      });
      return;
    }

    try {
      setActionLoading(true);
      
      // Convert string values to enum indices
      const gameTypeIndex = gameTypes.findIndex(type => type.value === selectedGameType);
      const difficultyIndex = difficultyLevels.findIndex(level => level.value === selectedDifficulty);
      
      await critterCraftAPI.startGame(
        selectedPet,
        gameTypeIndex,
        difficultyIndex
      );
      
      notification.success({
        message: 'Game started',
        description: `You've started a ${selectedDifficulty} ${gameTypes.find(type => type.value === selectedGameType).label} game!`,
      });
      
      setStartGameModalVisible(false);
      fetchGames();
    } catch (error) {
      console.error('Failed to start game:', error);
      notification.error({
        message: 'Failed to start game',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Submit a score for a game
  const handleSubmitScore = async () => {
    if (!selectedGameId) {
      notification.warning({
        message: 'No game selected',
        description: 'Please select a game to submit a score for.',
      });
      return;
    }

    try {
      setActionLoading(true);
      
      await critterCraftAPI.submitScore(selectedGameId, score);
      
      notification.success({
        message: 'Score submitted',
        description: `You've submitted a score of ${score} for your game!`,
      });
      
      setSubmitScoreModalVisible(false);
      fetchGames();
    } catch (error) {
      console.error('Failed to submit score:', error);
      notification.error({
        message: 'Failed to submit score',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Cancel a game
  const handleCancelGame = async (gameId) => {
    try {
      setActionLoading(true);
      
      await critterCraftAPI.cancelGame(gameId);
      
      notification.success({
        message: 'Game canceled',
        description: 'The game has been canceled successfully.',
      });
      
      fetchGames();
    } catch (error) {
      console.error('Failed to cancel game:', error);
      notification.error({
        message: 'Failed to cancel game',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Open the submit score modal
  const openSubmitScoreModal = (gameId) => {
    setSelectedGameId(gameId);
    setSubmitScoreModalVisible(true);
  };

  // Columns for the active games table
  const activeGamesColumns = [
    {
      title: 'Game',
      dataIndex: 'game_type',
      key: 'game_type',
      render: (gameType) => {
        const game = gameTypes.find(type => type.value === gameType);
        return game ? game.label : gameType;
      },
    },
    {
      title: 'Pet',
      dataIndex: 'pet_id',
      key: 'pet_id',
      render: (petId) => {
        const pet = pets.find(p => p.id === petId);
        return pet ? pet.name : `Pet #${petId}`;
      },
    },
    {
      title: 'Difficulty',
      dataIndex: 'difficulty',
      key: 'difficulty',
      render: (difficulty) => {
        const level = difficultyLevels.find(level => level.value === difficulty);
        return level ? (
          <Tag color={
            difficulty === 'Easy' ? 'green' :
            difficulty === 'Medium' ? 'blue' :
            difficulty === 'Hard' ? 'orange' :
            'red'
          }>
            {level.label}
          </Tag>
        ) : difficulty;
      },
    },
    {
      title: 'Started',
      dataIndex: 'started_at',
      key: 'started_at',
      render: (startedAt) => new Date(startedAt).toLocaleString(),
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <div>
          <Button 
            type="primary" 
            icon={<CheckCircleOutlined />} 
            onClick={() => openSubmitScoreModal(record.id)}
            style={{ marginRight: 8 }}
          >
            Submit Score
          </Button>
          <Button 
            danger 
            icon={<CloseCircleOutlined />} 
            onClick={() => handleCancelGame(record.id)}
          >
            Cancel
          </Button>
        </div>
      ),
    },
  ];

  // Columns for the completed games table
  const completedGamesColumns = [
    {
      title: 'Game',
      dataIndex: 'game_type',
      key: 'game_type',
      render: (gameType) => {
        const game = gameTypes.find(type => type.value === gameType);
        return game ? game.label : gameType;
      },
    },
    {
      title: 'Pet',
      dataIndex: 'pet_id',
      key: 'pet_id',
      render: (petId) => {
        const pet = pets.find(p => p.id === petId);
        return pet ? pet.name : `Pet #${petId}`;
      },
    },
    {
      title: 'Difficulty',
      dataIndex: 'difficulty',
      key: 'difficulty',
      render: (difficulty) => {
        const level = difficultyLevels.find(level => level.value === difficulty);
        return level ? (
          <Tag color={
            difficulty === 'Easy' ? 'green' :
            difficulty === 'Medium' ? 'blue' :
            difficulty === 'Hard' ? 'orange' :
            'red'
          }>
            {level.label}
          </Tag>
        ) : difficulty;
      },
    },
    {
      title: 'Score',
      dataIndex: 'score',
      key: 'score',
      render: (score) => <strong>{score}</strong>,
    },
    {
      title: 'XP Reward',
      dataIndex: 'experience_reward',
      key: 'experience_reward',
      render: (xp) => <Tag color="purple">{xp} XP</Tag>,
    },
    {
      title: 'BITS Reward',
      dataIndex: 'currency_reward',
      key: 'currency_reward',
      render: (bits) => <Tag color="gold">{bits} BITS</Tag>,
    },
    {
      title: 'Completed',
      dataIndex: 'completed_at',
      key: 'completed_at',
      render: (completedAt) => new Date(completedAt).toLocaleString(),
    },
  ];

  return (
    <div>
      <Card 
        title="Mini-Games" 
        extra={
          <Button 
            type="primary" 
            icon={<PlayCircleOutlined />} 
            onClick={() => setStartGameModalVisible(true)}
          >
            Start New Game
          </Button>
        }
        style={{ width: '100%', marginBottom: 16 }}
      >
        <Tabs defaultActiveKey="active">
          <TabPane tab="Active Games" key="active">
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading games...</p>
              </div>
            ) : (
              <Table 
                dataSource={activeGames} 
                columns={activeGamesColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={false}
                locale={{ emptyText: 'No active games. Start a new game to train your pet!' }}
              />
            )}
          </TabPane>
          <TabPane tab="Completed Games" key="completed">
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading games...</p>
              </div>
            ) : (
              <Table 
                dataSource={completedGames} 
                columns={completedGamesColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={{ pageSize: 5 }}
                locale={{ emptyText: 'No completed games yet. Complete a game to see your rewards!' }}
              />
            )}
          </TabPane>
        </Tabs>
      </Card>

      {/* Game Types Information */}
      <Card title="Game Types" style={{ width: '100%', marginBottom: 16 }}>
        <Row gutter={[16, 16]}>
          <Col span={8}>
            <Card>
              <Statistic
                title="Logic Leaper"
                value="Intelligence"
                prefix={<BulbOutlined style={{ color: '#722ed1' }} />}
                valueStyle={{ color: '#722ed1' }}
              />
              <p>A puzzle game that challenges your pet's problem-solving abilities. Trains Intelligence stat.</p>
            </Card>
          </Col>
          <Col span={8}>
            <Card>
              <Statistic
                title="Aura Weaving"
                value="Charisma"
                prefix={<TrophyOutlined style={{ color: '#fa8c16' }} />}
                valueStyle={{ color: '#fa8c16' }}
              />
              <p>A rhythm and pattern-matching game that enhances your pet's social skills. Trains Charisma stat.</p>
            </Card>
          </Col>
          <Col span={8}>
            <Card>
              <Statistic
                title="Habitat Dash"
                value="Agility"
                prefix={<RocketOutlined style={{ color: '#13c2c2' }} />}
                valueStyle={{ color: '#13c2c2' }}
              />
              <p>An endless runner style game that tests your pet's reflexes and speed. Trains Agility stat.</p>
            </Card>
          </Col>
        </Row>
      </Card>

      {/* Start Game Modal */}
      <Modal
        title="Start New Game"
        visible={startGameModalVisible}
        onOk={handleStartGame}
        onCancel={() => setStartGameModalVisible(false)}
        confirmLoading={actionLoading}
      >
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Select Pet:</label>
          <Select
            style={{ width: '100%' }}
            placeholder="Select a pet"
            value={selectedPet}
            onChange={setSelectedPet}
          >
            {pets.map(pet => (
              <Option key={pet.id} value={pet.id}>{pet.name}</Option>
            ))}
          </Select>
        </div>
        
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Game Type:</label>
          <Select
            style={{ width: '100%' }}
            value={selectedGameType}
            onChange={setSelectedGameType}
          >
            {gameTypes.map(type => (
              <Option key={type.value} value={type.value}>
                {type.label} - {type.description}
              </Option>
            ))}
          </Select>
        </div>
        
        <div>
          <label style={{ display: 'block', marginBottom: 8 }}>Difficulty:</label>
          <Select
            style={{ width: '100%' }}
            value={selectedDifficulty}
            onChange={setSelectedDifficulty}
          >
            {difficultyLevels.map(level => (
              <Option key={level.value} value={level.value}>
                {level.label} - {level.multiplier} rewards
              </Option>
            ))}
          </Select>
        </div>
      </Modal>

      {/* Submit Score Modal */}
      <Modal
        title="Submit Game Score"
        visible={submitScoreModalVisible}
        onOk={handleSubmitScore}
        onCancel={() => setSubmitScoreModalVisible(false)}
        confirmLoading={actionLoading}
      >
        <div>
          <label style={{ display: 'block', marginBottom: 8 }}>Your Score:</label>
          <InputNumber
            style={{ width: '100%' }}
            min={1}
            max={10000}
            value={score}
            onChange={setScore}
          />
          <p style={{ marginTop: 8, color: '#8c8c8c' }}>
            <ClockCircleOutlined /> Higher scores will earn more XP and BITS rewards!
          </p>
        </div>
      </Modal>
    </div>
  );
};

export default MinigamesPanel;