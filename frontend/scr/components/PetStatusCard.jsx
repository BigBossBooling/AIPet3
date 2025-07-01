import React, { useState, useEffect } from 'react';
import { Card, Progress, Button, Row, Col, Tooltip, Badge, Spin, notification } from 'antd';
import { 
  HeartFilled, 
  ThunderboltFilled, 
  SmileFilled, 
  ExperimentFilled, 
  TeamFilled,
  TrophyFilled,
  RocketFilled,
  FireFilled,
  BulbFilled,
  HeartTwoTone
} from '@ant-design/icons';
import critterCraftAPI from '../crittercraft_api';

/**
 * PetStatusCard component displays the status of a pet and allows interactions
 */
const PetStatusCard = ({ petId }) => {
  const [loading, setLoading] = useState(true);
  const [pet, setPet] = useState(null);
  const [petStatus, setPetStatus] = useState(null);
  const [petNeeds, setPetNeeds] = useState(null);
  const [petStats, setPetStats] = useState(null);
  const [actionLoading, setActionLoading] = useState(false);

  // Fetch pet data on component mount
  useEffect(() => {
    const fetchPetData = async () => {
      try {
        setLoading(true);
        const [petData, statusData, needsData, statsData] = await Promise.all([
          critterCraftAPI.getPet(petId),
          critterCraftAPI.getPetStatus(petId),
          critterCraftAPI.getPetNeeds(petId),
          critterCraftAPI.getPetStats(petId)
        ]);
        
        setPet(petData);
        setPetStatus(statusData);
        setPetNeeds(needsData);
        setPetStats(statsData);
      } catch (error) {
        console.error('Failed to fetch pet data:', error);
        notification.error({
          message: 'Failed to fetch pet data',
          description: error.message,
        });
      } finally {
        setLoading(false);
      }
    };

    fetchPetData();
  }, [petId]);

  // Perform a care action
  const performCareAction = async (action) => {
    try {
      setActionLoading(true);
      
      let result;
      switch (action) {
        case 'feed':
          result = await critterCraftAPI.feedPet(petId);
          break;
        case 'rest':
          result = await critterCraftAPI.restPet(petId);
          break;
        case 'play':
          result = await critterCraftAPI.playWithPet(petId);
          break;
        case 'groom':
          result = await critterCraftAPI.groomPet(petId);
          break;
        default:
          throw new Error('Invalid action');
      }
      
      // Refresh pet data
      const [statusData, needsData] = await Promise.all([
        critterCraftAPI.getPetStatus(petId),
        critterCraftAPI.getPetNeeds(petId)
      ]);
      
      setPetStatus(statusData);
      setPetNeeds(needsData);
      
      notification.success({
        message: 'Action successful',
        description: `Your pet has been ${action === 'feed' ? 'fed' : 
                                         action === 'rest' ? 'rested' : 
                                         action === 'play' ? 'played with' : 
                                         'groomed'} successfully!`,
      });
    } catch (error) {
      console.error(`Failed to ${action} pet:`, error);
      notification.error({
        message: `Failed to ${action} pet`,
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Get mood color based on mood
  const getMoodColor = (mood) => {
    switch (mood) {
      case 'Happy':
        return '#52c41a';
      case 'Content':
        return '#1890ff';
      case 'Neutral':
        return '#faad14';
      case 'Sad':
        return '#fa8c16';
      case 'Distressed':
        return '#f5222d';
      default:
        return '#d9d9d9';
    }
  };

  // Get need status color
  const getNeedColor = (value) => {
    if (value >= 80) return '#52c41a';
    if (value >= 60) return '#1890ff';
    if (value >= 40) return '#faad14';
    if (value >= 20) return '#fa8c16';
    return '#f5222d';
  };

  if (loading) {
    return (
      <Card title="Pet Status" style={{ width: '100%', marginBottom: 16 }}>
        <div style={{ textAlign: 'center', padding: 24 }}>
          <Spin size="large" />
          <p style={{ marginTop: 16 }}>Loading pet data...</p>
        </div>
      </Card>
    );
  }

  if (!pet || !petStatus || !petNeeds || !petStats) {
    return (
      <Card title="Pet Status" style={{ width: '100%', marginBottom: 16 }}>
        <div style={{ textAlign: 'center', padding: 24 }}>
          <p>Pet data not found. Please make sure the pet exists and has been initialized.</p>
          <Button 
            type="primary" 
            onClick={() => critterCraftAPI.initializePetStatus(petId)}
            loading={actionLoading}
          >
            Initialize Pet Status
          </Button>
        </div>
      </Card>
    );
  }

  return (
    <Card 
      title={
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <span>{pet.name}</span>
          <Badge 
            count={petStatus.mood} 
            style={{ backgroundColor: getMoodColor(petStatus.mood) }}
          />
        </div>
      }
      style={{ width: '100%', marginBottom: 16 }}
    >
      {/* Pet Needs Section */}
      <h3>Needs</h3>
      <Row gutter={[16, 16]}>
        <Col span={12}>
          <Tooltip title="Hunger">
            <div>
              <HeartFilled style={{ color: getNeedColor(petNeeds.hunger) }} /> Hunger
              <Progress 
                percent={petNeeds.hunger} 
                strokeColor={getNeedColor(petNeeds.hunger)} 
                size="small" 
              />
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Energy">
            <div>
              <ThunderboltFilled style={{ color: getNeedColor(petNeeds.energy) }} /> Energy
              <Progress 
                percent={petNeeds.energy} 
                strokeColor={getNeedColor(petNeeds.energy)} 
                size="small" 
              />
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Happiness">
            <div>
              <SmileFilled style={{ color: getNeedColor(petNeeds.happiness) }} /> Happiness
              <Progress 
                percent={petNeeds.happiness} 
                strokeColor={getNeedColor(petNeeds.happiness)} 
                size="small" 
              />
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Hygiene">
            <div>
              <ExperimentFilled style={{ color: getNeedColor(petNeeds.hygiene) }} /> Hygiene
              <Progress 
                percent={petNeeds.hygiene} 
                strokeColor={getNeedColor(petNeeds.hygiene)} 
                size="small" 
              />
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Social">
            <div>
              <TeamFilled style={{ color: getNeedColor(petNeeds.social) }} /> Social
              <Progress 
                percent={petNeeds.social} 
                strokeColor={getNeedColor(petNeeds.social)} 
                size="small" 
              />
            </div>
          </Tooltip>
        </Col>
      </Row>

      {/* Pet Stats Section */}
      <h3 style={{ marginTop: 16 }}>Stats</h3>
      <Row gutter={[16, 16]}>
        <Col span={12}>
          <Tooltip title="Strength">
            <div>
              <TrophyFilled style={{ color: '#722ed1' }} /> Strength: {petStats.strength}
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Agility">
            <div>
              <RocketFilled style={{ color: '#13c2c2' }} /> Agility: {petStats.agility}
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Intelligence">
            <div>
              <BulbFilled style={{ color: '#1890ff' }} /> Intelligence: {petStats.intelligence}
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Vitality">
            <div>
              <HeartTwoTone twoToneColor="#eb2f96" /> Vitality: {petStats.vitality}
            </div>
          </Tooltip>
        </Col>
        <Col span={12}>
          <Tooltip title="Charisma">
            <div>
              <FireFilled style={{ color: '#fa8c16' }} /> Charisma: {petStats.charisma}
            </div>
          </Tooltip>
        </Col>
      </Row>

      {/* Care Actions Section */}
      <h3 style={{ marginTop: 16 }}>Care Actions</h3>
      <Row gutter={[16, 16]}>
        <Col span={6}>
          <Button 
            type="primary" 
            icon={<HeartFilled />} 
            onClick={() => performCareAction('feed')}
            loading={actionLoading}
            block
          >
            Feed
          </Button>
        </Col>
        <Col span={6}>
          <Button 
            type="primary" 
            icon={<ThunderboltFilled />} 
            onClick={() => performCareAction('rest')}
            loading={actionLoading}
            block
          >
            Rest
          </Button>
        </Col>
        <Col span={6}>
          <Button 
            type="primary" 
            icon={<SmileFilled />} 
            onClick={() => performCareAction('play')}
            loading={actionLoading}
            block
          >
            Play
          </Button>
        </Col>
        <Col span={6}>
          <Button 
            type="primary" 
            icon={<ExperimentFilled />} 
            onClick={() => performCareAction('groom')}
            loading={actionLoading}
            block
          >
            Groom
          </Button>
        </Col>
      </Row>

      {/* Last Interaction Times */}
      <div style={{ marginTop: 16, fontSize: '12px', color: '#8c8c8c' }}>
        <p>Last Fed: {new Date(petStatus.last_fed).toLocaleString()}</p>
        <p>Last Rested: {new Date(petStatus.last_rested).toLocaleString()}</p>
        <p>Last Played: {new Date(petStatus.last_played).toLocaleString()}</p>
        <p>Last Groomed: {new Date(petStatus.last_groomed).toLocaleString()}</p>
        <p>Last Socialized: {new Date(petStatus.last_socialized).toLocaleString()}</p>
      </div>
    </Card>
  );
};

export default PetStatusCard;
