import React, { useState, useEffect } from 'react';
import { Layout, Menu, Button, Spin, notification, Avatar, Typography, Tabs, Card, Modal } from 'antd';
import {
  UserOutlined,
  HomeOutlined,
  HeartOutlined,
  TrophyOutlined,
  DollarOutlined,
  TeamOutlined,
  LogoutOutlined,
  LoginOutlined,
  PlusOutlined,
} from '@ant-design/icons';
import PetStatusCard from './components/PetStatusCard';
import MinigamesPanel from './components/MinigamesPanel';
import JobsPanel from './components/JobsPanel';
import DaycarePanel from './components/DaycarePanel';
import critterCraftAPI from './crittercraft_api';

const { Header, Content, Footer, Sider } = Layout;
const { Title, Text } = Typography;
const { TabPane } = Tabs;

/**
 * Main App component
 */
const App = () => {
  const [collapsed, setCollapsed] = useState(false);
  const [loading, setLoading] = useState(true);
  const [connecting, setConnecting] = useState(true);
  const [connected, setConnected] = useState(false);
  const [accounts, setAccounts] = useState([]);
  const [selectedAccount, setSelectedAccount] = useState(null);
  const [balance, setBalance] = useState('0');
  const [pets, setPets] = useState([]);
  const [selectedPet, setSelectedPet] = useState(null);
  const [activeTab, setActiveTab] = useState('home');
  const [mintModalVisible, setMintModalVisible] = useState(false);
  const [petName, setPetName] = useState('');
  const [petDescription, setPetDescription] = useState('');
  const [petType, setPetType] = useState(0);
  const [mintLoading, setMintLoading] = useState(false);

  // Connect to the blockchain on component mount
  useEffect(() => {
    const connectToBlockchain = async () => {
      try {
        setConnecting(true);
        const success = await critterCraftAPI.connect();
        setConnected(success);
      } catch (error) {
        console.error('Failed to connect to blockchain:', error);
        notification.error({
          message: 'Connection Failed',
          description: 'Failed to connect to the CritterCraft blockchain. Please try again later.',
        });
        setConnected(false);
      } finally {
        setConnecting(false);
      }
    };

    connectToBlockchain();
  }, []);

  // Connect wallet and fetch data when connected
  const connectWallet = async () => {
    try {
      setLoading(true);
      const walletAccounts = await critterCraftAPI.connectWallet();
      setAccounts(walletAccounts);
      
      if (walletAccounts.length > 0) {
        setSelectedAccount(walletAccounts[0]);
        critterCraftAPI.setAccount(walletAccounts[0]);
        
        // Fetch balance
        const accountBalance = await critterCraftAPI.getBalance(walletAccounts[0].address);
        setBalance(accountBalance);
        
        // Fetch pets
        const petIds = await critterCraftAPI.getPetsByOwner(walletAccounts[0].address);
        const petsPromises = petIds.map(id => critterCraftAPI.getPet(id));
        const petsData = await Promise.all(petsPromises);
        setPets(petsData);
        
        if (petsData.length > 0) {
          setSelectedPet(petsData[0].id);
        }
      }
    } catch (error) {
      console.error('Failed to connect wallet:', error);
      notification.error({
        message: 'Wallet Connection Failed',
        description: error.message,
      });
    } finally {
      setLoading(false);
    }
  };

  // Disconnect wallet
  const disconnectWallet = () => {
    setSelectedAccount(null);
    setPets([]);
    setSelectedPet(null);
    setBalance('0');
  };

  // Mint a new pet
  const handleMintPet = async () => {
    if (!petName) {
      notification.warning({
        message: 'Name Required',
        description: 'Please enter a name for your pet.',
      });
      return;
    }

    try {
      setMintLoading(true);
      
      await critterCraftAPI.mintPet(petName, petDescription, petType);
      
      notification.success({
        message: 'Pet Minted',
        description: `Your new pet "${petName}" has been minted successfully!`,
      });
      
      // Refresh pets
      const petIds = await critterCraftAPI.getPetsByOwner(selectedAccount.address);
      const petsPromises = petIds.map(id => critterCraftAPI.getPet(id));
      const petsData = await Promise.all(petsPromises);
      setPets(petsData);
      
      // Refresh balance
      const accountBalance = await critterCraftAPI.getBalance(selectedAccount.address);
      setBalance(accountBalance);
      
      setMintModalVisible(false);
      setPetName('');
      setPetDescription('');
      setPetType(0);
    } catch (error) {
      console.error('Failed to mint pet:', error);
      notification.error({
        message: 'Mint Failed',
        description: error.message,
      });
    } finally {
      setMintLoading(false);
    }
  };

  // Render connection screen
  if (connecting) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh', flexDirection: 'column' }}>
        <Spin size="large" />
        <Title level={3} style={{ marginTop: 24 }}>Connecting to CritterCraft...</Title>
      </div>
    );
  }

  // Render connection error screen
  if (!connected) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh', flexDirection: 'column' }}>
        <Title level={3}>Connection Failed</Title>
        <Text>Could not connect to the CritterCraft blockchain.</Text>
        <Button type="primary" onClick={() => window.location.reload()} style={{ marginTop: 16 }}>
          Retry Connection
        </Button>
      </div>
    );
  }

  // Render wallet connection screen
  if (!selectedAccount) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh', flexDirection: 'column' }}>
        <Title level={2}>Welcome to CritterCraft</Title>
        <Text style={{ fontSize: 16, marginBottom: 24 }}>Connect your wallet to start your pet adventure!</Text>
        <Button 
          type="primary" 
          icon={<LoginOutlined />} 
          onClick={connectWallet} 
          loading={loading}
          size="large"
        >
          Connect Wallet
        </Button>
      </div>
    );
  }

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider collapsible collapsed={collapsed} onCollapse={setCollapsed}>
        <div style={{ height: 32, margin: 16, background: 'rgba(255, 255, 255, 0.2)', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
          <Title level={5} style={{ color: 'white', margin: 0 }}>CritterCraft</Title>
        </div>
        <Menu theme="dark" selectedKeys={[activeTab]} mode="inline" onSelect={({ key }) => setActiveTab(key)}>
          <Menu.Item key="home" icon={<HomeOutlined />}>
            Home
          </Menu.Item>
          <Menu.Item key="pet" icon={<HeartOutlined />} disabled={!selectedPet}>
            Pet Status
          </Menu.Item>
          <Menu.Item key="minigames" icon={<TrophyOutlined />} disabled={!selectedPet}>
            Mini-Games
          </Menu.Item>
          <Menu.Item key="jobs" icon={<DollarOutlined />} disabled={!selectedPet}>
            Jobs
          </Menu.Item>
          <Menu.Item key="daycare" icon={<TeamOutlined />}>
            Daycare
          </Menu.Item>
          <Menu.Item key="logout" icon={<LogoutOutlined />} onClick={disconnectWallet}>
            Disconnect
          </Menu.Item>
        </Menu>
      </Sider>
      <Layout className="site-layout">
        <Header style={{ padding: '0 16px', background: '#fff', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <div style={{ display: 'flex', alignItems: 'center' }}>
            <Avatar icon={<UserOutlined />} />
            <Text style={{ marginLeft: 8 }}>{selectedAccount.meta.name}</Text>
          </div>
          <div>
            <Text strong style={{ marginRight: 16 }}>Balance: {balance}</Text>
            <Button 
              type="primary" 
              icon={<PlusOutlined />} 
              onClick={() => setMintModalVisible(true)}
            >
              Mint New Pet
            </Button>
          </div>
        </Header>
        <Content style={{ margin: '16px' }}>
          {activeTab === 'home' && (
            <div>
              <Title level={2}>Welcome to CritterCraft</Title>
              <Text style={{ fontSize: 16, marginBottom: 16, display: 'block' }}>
                Your virtual pet adventure on the blockchain!
              </Text>
              
              <Title level={3} style={{ marginTop: 24 }}>Your Pets</Title>
              {loading ? (
                <div style={{ textAlign: 'center', padding: 24 }}>
                  <Spin size="large" />
                  <p style={{ marginTop: 16 }}>Loading pets...</p>
                </div>
              ) : pets.length === 0 ? (
                <Card style={{ textAlign: 'center', padding: 24 }}>
                  <p>You don't have any pets yet.</p>
                  <Button 
                    type="primary" 
                    icon={<PlusOutlined />} 
                    onClick={() => setMintModalVisible(true)}
                  >
                    Mint Your First Pet
                  </Button>
                </Card>
              ) : (
                <Tabs 
                  activeKey={selectedPet ? selectedPet.toString() : undefined} 
                  onChange={(key) => setSelectedPet(parseInt(key))}
                >
                  {pets.map(pet => (
                    <TabPane tab={pet.name} key={pet.id}>
                      <PetStatusCard petId={pet.id} />
                    </TabPane>
                  ))}
                </Tabs>
              )}
            </div>
          )}

          {activeTab === 'pet' && selectedPet && (
            <PetStatusCard petId={selectedPet} />
          )}

          {activeTab === 'minigames' && selectedPet && (
            <MinigamesPanel pets={pets} />
          )}

          {activeTab === 'jobs' && selectedPet && (
            <JobsPanel pets={pets} />
          )}

          {activeTab === 'daycare' && (
            <DaycarePanel pets={pets} />
          )}
        </Content>
        <Footer style={{ textAlign: 'center' }}>
          CritterCraft ©2023 - A blockchain-based virtual pet ecosystem
        </Footer>
      </Layout>

      {/* Mint Pet Modal */}
      <Modal
        title="Mint New Pet"
        visible={mintModalVisible}
        onOk={handleMintPet}
        onCancel={() => setMintModalVisible(false)}
        confirmLoading={mintLoading}
      >
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Pet Name:</label>
          <input
            style={{ width: '100%', padding: '8px', borderRadius: '4px', border: '1px solid #d9d9d9' }}
            placeholder="Enter pet name"
            value={petName}
            onChange={(e) => setPetName(e.target.value)}
            maxLength={32}
          />
        </div>
        
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Description:</label>
          <textarea
            style={{ width: '100%', padding: '8px', borderRadius: '4px', border: '1px solid #d9d9d9' }}
            placeholder="Enter pet description"
            value={petDescription}
            onChange={(e) => setPetDescription(e.target.value)}
            maxLength={256}
            rows={4}
          />
        </div>
        
        <div>
          <label style={{ display: 'block', marginBottom: 8 }}>Pet Type:</label>
          <select
            style={{ width: '100%', padding: '8px', borderRadius: '4px', border: '1px solid #d9d9d9' }}
            value={petType}
            onChange={(e) => setPetType(parseInt(e.target.value))}
          >
            <option value={0}>Terrestrial</option>
            <option value={1}>Aquatic</option>
            <option value={2}>Aerial</option>
            <option value={3}>Ethereal</option>
          </select>
        </div>
      </Modal>
    </Layout>
  );
};

export default App;