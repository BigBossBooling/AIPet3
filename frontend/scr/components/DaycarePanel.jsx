import React, { useState, useEffect } from 'react';
import { Card, Button, Select, Table, Tag, Modal, Input, InputNumber, Spin, notification, Tabs, Statistic, Row, Col, List, Avatar, Dropdown, Menu } from 'antd';
import { 
  HomeOutlined, 
  PlusOutlined, 
  EditOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  HeartOutlined,
  ThunderboltOutlined,
  SmileOutlined,
  ExperimentOutlined,
  TeamOutlined,
  SettingOutlined,
  DollarOutlined
} from '@ant-design/icons';
import critterCraftAPI from '../crittercraft_api';

const { Option } = Select;
const { TabPane } = Tabs;
const { TextArea } = Input;

/**
 * DaycarePanel component for managing daycares and listings
 */
const DaycarePanel = ({ pets = [] }) => {
  const [loading, setLoading] = useState(true);
  const [myDaycares, setMyDaycares] = useState([]);
  const [availableDaycares, setAvailableDaycares] = useState([]);
  const [myListings, setMyListings] = useState([]);
  const [caregiverListings, setCaregiverListings] = useState([]);
  
  const [createDaycareModalVisible, setCreateDaycareModalVisible] = useState(false);
  const [createListingModalVisible, setCreateListingModalVisible] = useState(false);
  const [performCareModalVisible, setPerformCareModalVisible] = useState(false);
  
  const [selectedDaycare, setSelectedDaycare] = useState(null);
  const [selectedPet, setSelectedPet] = useState(null);
  const [selectedListing, setSelectedListing] = useState(null);
  const [selectedCareAction, setSelectedCareAction] = useState('Feed');
  const [selectedTargetPet, setSelectedTargetPet] = useState(null);
  
  const [daycareName, setDaycareName] = useState('');
  const [daycareDescription, setDaycareDescription] = useState('');
  const [feePerBlock, setFeePerBlock] = useState(1);
  const [listingDuration, setListingDuration] = useState(1000);
  
  const [actionLoading, setActionLoading] = useState(false);
  const [refreshing, setRefreshing] = useState(false);

  // Care action options
  const careActions = [
    { value: 'Feed', label: 'Feed', icon: <HeartOutlined /> },
    { value: 'Rest', label: 'Rest', icon: <ThunderboltOutlined /> },
    { value: 'Play', label: 'Play', icon: <SmileOutlined /> },
    { value: 'Groom', label: 'Groom', icon: <ExperimentOutlined /> },
    { value: 'Socialize', label: 'Socialize', icon: <TeamOutlined /> },
  ];

  // Fetch daycares and listings on component mount
  useEffect(() => {
    fetchDaycares();
    fetchListings();
  }, []);

  // Fetch daycares
  const fetchDaycares = async () => {
    try {
      setRefreshing(true);
      
      // Fetch daycares owned by the current user
      const myDaycareIds = await critterCraftAPI.getDaycaresByOwner();
      const myDaycaresPromises = myDaycareIds.map(id => critterCraftAPI.getDaycare(id));
      const myDaycaresData = await Promise.all(myDaycaresPromises);
      
      // Fetch all available daycares (this is a placeholder - in a real app, you'd have an API for this)
      // For now, we'll just use a mock
      const availableDaycaresData = []; // Mock data would go here
      
      setMyDaycares(myDaycaresData);
      setAvailableDaycares(availableDaycaresData);
    } catch (error) {
      console.error('Failed to fetch daycares:', error);
      notification.error({
        message: 'Failed to fetch daycares',
        description: error.message,
      });
    } finally {
      setRefreshing(false);
    }
  };

  // Fetch listings
  const fetchListings = async () => {
    try {
      setRefreshing(true);
      
      // Fetch listings for pets owned by the current user
      // This is a placeholder - in a real app, you'd have an API for this
      const myListingsData = []; // Mock data would go here
      
      // Fetch listings where the current user is the caregiver
      // This is a placeholder - in a real app, you'd have an API for this
      const caregiverListingsData = []; // Mock data would go here
      
      setMyListings(myListingsData);
      setCaregiverListings(caregiverListingsData);
    } catch (error) {
      console.error('Failed to fetch listings:', error);
      notification.error({
        message: 'Failed to fetch listings',
        description: error.message,
      });
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  };

  // Create a new daycare
  const handleCreateDaycare = async () => {
    try {
      setActionLoading(true);
      
      await critterCraftAPI.createDaycare(
        daycareName,
        daycareDescription,
        feePerBlock
      );
      
      notification.success({
        message: 'Daycare created',
        description: `Your daycare "${daycareName}" has been created successfully!`,
      });
      
      setCreateDaycareModalVisible(false);
      setDaycareName('');
      setDaycareDescription('');
      setFeePerBlock(1);
      fetchDaycares();
    } catch (error) {
      console.error('Failed to create daycare:', error);
      notification.error({
        message: 'Failed to create daycare',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Create a new listing
  const handleCreateListing = async () => {
    if (!selectedDaycare) {
      notification.warning({
        message: 'No daycare selected',
        description: 'Please select a daycare for the listing.',
      });
      return;
    }

    if (!selectedPet) {
      notification.warning({
        message: 'No pet selected',
        description: 'Please select a pet for the listing.',
      });
      return;
    }

    try {
      setActionLoading(true);
      
      await critterCraftAPI.createListing(
        selectedDaycare,
        selectedPet,
        listingDuration
      );
      
      notification.success({
        message: 'Listing created',
        description: 'Your pet has been listed in the daycare successfully!',
      });
      
      setCreateListingModalVisible(false);
      setSelectedDaycare(null);
      setSelectedPet(null);
      setListingDuration(1000);
      fetchListings();
    } catch (error) {
      console.error('Failed to create listing:', error);
      notification.error({
        message: 'Failed to create listing',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Accept a listing as a caregiver
  const handleAcceptListing = async (listingId) => {
    try {
      setActionLoading(true);
      
      await critterCraftAPI.acceptListing(listingId);
      
      notification.success({
        message: 'Listing accepted',
        description: 'You are now the caregiver for this pet!',
      });
      
      fetchListings();
    } catch (error) {
      console.error('Failed to accept listing:', error);
      notification.error({
        message: 'Failed to accept listing',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Complete a listing
  const handleCompleteListing = async (listingId) => {
    try {
      setActionLoading(true);
      
      await critterCraftAPI.completeListing(listingId);
      
      notification.success({
        message: 'Listing completed',
        description: 'The listing has been completed successfully!',
      });
      
      fetchListings();
    } catch (error) {
      console.error('Failed to complete listing:', error);
      notification.error({
        message: 'Failed to complete listing',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Cancel a listing
  const handleCancelListing = async (listingId) => {
    try {
      setActionLoading(true);
      
      await critterCraftAPI.cancelListing(listingId);
      
      notification.success({
        message: 'Listing canceled',
        description: 'The listing has been canceled successfully.',
      });
      
      fetchListings();
    } catch (error) {
      console.error('Failed to cancel listing:', error);
      notification.error({
        message: 'Failed to cancel listing',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Perform a care action
  const handlePerformCareAction = async () => {
    if (!selectedListing) {
      notification.warning({
        message: 'No listing selected',
        description: 'Please select a listing to perform care action.',
      });
      return;
    }

    try {
      setActionLoading(true);
      
      // Convert string values to enum indices
      const actionIndex = careActions.findIndex(action => action.value === selectedCareAction);
      
      await critterCraftAPI.performCareAction(
        selectedListing,
        actionIndex,
        selectedCareAction === 'Socialize' ? selectedTargetPet : null
      );
      
      notification.success({
        message: 'Care action performed',
        description: `You've performed the ${selectedCareAction} action successfully!`,
      });
      
      setPerformCareModalVisible(false);
      setSelectedListing(null);
      setSelectedCareAction('Feed');
      setSelectedTargetPet(null);
    } catch (error) {
      console.error('Failed to perform care action:', error);
      notification.error({
        message: 'Failed to perform care action',
        description: error.message,
      });
    } finally {
      setActionLoading(false);
    }
  };

  // Open the perform care modal
  const openPerformCareModal = (listingId) => {
    setSelectedListing(listingId);
    setPerformCareModalVisible(true);
  };

  // Columns for the my daycares table
  const myDaycaresColumns = [
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: 'Description',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
    },
    {
      title: 'Fee Per Block',
      dataIndex: 'fee_per_block',
      key: 'fee_per_block',
      render: (fee) => <Tag color="gold">{fee} BITS</Tag>,
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status) => (
        <Tag color={status === 'Active' ? 'green' : 'red'}>
          {status}
        </Tag>
      ),
    },
    {
      title: 'Reputation',
      dataIndex: 'reputation',
      key: 'reputation',
      render: (reputation) => <Tag color="blue">{reputation}</Tag>,
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <Dropdown
          overlay={
            <Menu>
              <Menu.Item key="1" icon={<EditOutlined />}>
                Edit Daycare
              </Menu.Item>
              <Menu.Item key="2" icon={<PlusOutlined />} onClick={() => {
                setSelectedDaycare(record.id);
                setCreateListingModalVisible(true);
              }}>
                Create Listing
              </Menu.Item>
              <Menu.Item key="3" icon={<SettingOutlined />}>
                Manage Listings
              </Menu.Item>
            </Menu>
          }
        >
          <Button icon={<SettingOutlined />}>Actions</Button>
        </Dropdown>
      ),
    },
  ];

  // Columns for the available daycares table
  const availableDaycaresColumns = [
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: 'Owner',
      dataIndex: 'owner',
      key: 'owner',
      render: (owner) => `${owner.slice(0, 6)}...${owner.slice(-4)}`,
    },
    {
      title: 'Description',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
    },
    {
      title: 'Fee Per Block',
      dataIndex: 'fee_per_block',
      key: 'fee_per_block',
      render: (fee) => <Tag color="gold">{fee} BITS</Tag>,
    },
    {
      title: 'Reputation',
      dataIndex: 'reputation',
      key: 'reputation',
      render: (reputation) => <Tag color="blue">{reputation}</Tag>,
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <Button 
          type="primary" 
          icon={<PlusOutlined />} 
          onClick={() => {
            setSelectedDaycare(record.id);
            setCreateListingModalVisible(true);
          }}
        >
          Create Listing
        </Button>
      ),
    },
  ];

  // Columns for the my listings table
  const myListingsColumns = [
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
      title: 'Daycare',
      dataIndex: 'daycare_id',
      key: 'daycare_id',
      render: (daycareId) => {
        const daycare = [...myDaycares, ...availableDaycares].find(d => d.id === daycareId);
        return daycare ? daycare.name : `Daycare #${daycareId}`;
      },
    },
    {
      title: 'Caregiver',
      dataIndex: 'caregiver',
      key: 'caregiver',
      render: (caregiver) => caregiver ? `${caregiver.slice(0, 6)}...${caregiver.slice(-4)}` : 'None',
    },
    {
      title: 'Fee',
      dataIndex: 'total_fee',
      key: 'total_fee',
      render: (fee) => <Tag color="gold">{fee} BITS</Tag>,
    },
    {
      title: 'Duration',
      dataIndex: 'duration',
      key: 'duration',
      render: (duration) => `${duration} blocks`,
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status) => (
        <Tag color={
          status === 'Active' ? 'green' :
          status === 'Completed' ? 'blue' :
          'red'
        }>
          {status}
        </Tag>
      ),
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <div>
          <Button 
            type="primary" 
            icon={<CheckCircleOutlined />} 
            onClick={() => handleCompleteListing(record.id)}
            disabled={record.status !== 'Active'}
            style={{ marginRight: 8 }}
          >
            Complete
          </Button>
          <Button 
            danger 
            icon={<CloseCircleOutlined />} 
            onClick={() => handleCancelListing(record.id)}
            disabled={record.status !== 'Active'}
          >
            Cancel
          </Button>
        </div>
      ),
    },
  ];

  // Columns for the caregiver listings table
  const caregiverListingsColumns = [
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
      title: 'Owner',
      dataIndex: 'owner',
      key: 'owner',
      render: (owner) => `${owner.slice(0, 6)}...${owner.slice(-4)}`,
    },
    {
      title: 'Daycare',
      dataIndex: 'daycare_id',
      key: 'daycare_id',
      render: (daycareId) => {
        const daycare = [...myDaycares, ...availableDaycares].find(d => d.id === daycareId);
        return daycare ? daycare.name : `Daycare #${daycareId}`;
      },
    },
    {
      title: 'Fee',
      dataIndex: 'caregiver_fee',
      key: 'caregiver_fee',
      render: (fee) => <Tag color="gold">{fee} BITS</Tag>,
    },
    {
      title: 'Last Care',
      dataIndex: 'last_care_action',
      key: 'last_care_action',
      render: (lastCare) => lastCare ? new Date(lastCare).toLocaleString() : 'Never',
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status) => (
        <Tag color={status === 'Active' ? 'green' : 'blue'}>
          {status}
        </Tag>
      ),
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <Button 
          type="primary" 
          icon={<HeartOutlined />} 
          onClick={() => openPerformCareModal(record.id)}
          disabled={record.status !== 'Active'}
        >
          Perform Care
        </Button>
      ),
    },
  ];

  return (
    <div>
      <Tabs defaultActiveKey="daycares">
        <TabPane tab="Daycares" key="daycares">
          <Card 
            title="My Daycares" 
            extra={
              <Button 
                type="primary" 
                icon={<PlusOutlined />} 
                onClick={() => setCreateDaycareModalVisible(true)}
              >
                Create Daycare
              </Button>
            }
            style={{ width: '100%', marginBottom: 16 }}
          >
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading daycares...</p>
              </div>
            ) : (
              <Table 
                dataSource={myDaycares} 
                columns={myDaycaresColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={false}
                locale={{ emptyText: 'No daycares yet. Create a daycare to earn BITS!' }}
              />
            )}
          </Card>

          <Card 
            title="Available Daycares" 
            style={{ width: '100%' }}
          >
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading daycares...</p>
              </div>
            ) : (
              <Table 
                dataSource={availableDaycares} 
                columns={availableDaycaresColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={{ pageSize: 5 }}
                locale={{ emptyText: 'No available daycares found.' }}
              />
            )}
          </Card>
        </TabPane>

        <TabPane tab="Listings" key="listings">
          <Card 
            title="My Pet Listings" 
            extra={
              <Button 
                type="primary" 
                icon={<PlusOutlined />} 
                onClick={() => setCreateListingModalVisible(true)}
                disabled={myDaycares.length === 0 && availableDaycares.length === 0}
              >
                Create Listing
              </Button>
            }
            style={{ width: '100%', marginBottom: 16 }}
          >
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading listings...</p>
              </div>
            ) : (
              <Table 
                dataSource={myListings} 
                columns={myListingsColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={false}
                locale={{ emptyText: 'No pet listings yet. Create a listing to place your pet in a daycare!' }}
              />
            )}
          </Card>

          <Card 
            title="Caregiver Listings" 
            style={{ width: '100%' }}
          >
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading listings...</p>
              </div>
            ) : (
              <Table 
                dataSource={caregiverListings} 
                columns={caregiverListingsColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={{ pageSize: 5 }}
                locale={{ emptyText: 'No caregiver listings yet. Accept a listing to become a caregiver!' }}
              />
            )}
          </Card>
        </TabPane>

        <TabPane tab="How It Works" key="info">
          <Card title="Daycare System" style={{ width: '100%', marginBottom: 16 }}>
            <Row gutter={[16, 16]}>
              <Col span={8}>
                <Card>
                  <Statistic
                    title="Create a Daycare"
                    value="Step 1"
                    prefix={<HomeOutlined style={{ color: '#1890ff' }} />}
                    valueStyle={{ color: '#1890ff' }}
                  />
                  <p>Create your own daycare facility where other players can leave their pets. Set your own fees and build your reputation.</p>
                </Card>
              </Col>
              <Col span={8}>
                <Card>
                  <Statistic
                    title="List Your Pet"
                    value="Step 2"
                    prefix={<PlusOutlined style={{ color: '#52c41a' }} />}
                    valueStyle={{ color: '#52c41a' }}
                  />
                  <p>List your pet in a daycare when you'll be offline. Specify the duration and pay a fee based on the daycare's rates.</p>
                </Card>
              </Col>
              <Col span={8}>
                <Card>
                  <Statistic
                    title="Provide Care"
                    value="Step 3"
                    prefix={<HeartOutlined style={{ color: '#eb2f96' }} />}
                    valueStyle={{ color: '#eb2f96' }}
                  />
                  <p>Become a caregiver for other players' pets. Perform care actions to maintain their needs and earn BITS and reputation.</p>
                </Card>
              </Col>
            </Row>
          </Card>

          <Card title="Economic Model" style={{ width: '100%' }}>
            <List
              itemLayout="horizontal"
              dataSource={[
                {
                  title: 'Fee Structure',
                  description: 'Daycare owners set a fee per block. The total fee is calculated based on the listing duration.',
                  icon: <DollarOutlined style={{ color: '#faad14' }} />,
                },
                {
                  title: 'Platform Fee',
                  description: '5% of the total fee goes to the platform treasury to support ongoing development.',
                  icon: <DollarOutlined style={{ color: '#faad14' }} />,
                },
                {
                  title: 'Caregiver Earnings',
                  description: '95% of the total fee goes to the caregiver who accepts the listing and provides care.',
                  icon: <DollarOutlined style={{ color: '#faad14' }} />,
                },
                {
                  title: 'Reputation System',
                  description: 'Caregivers earn reputation points for successfully completing listings, which increases visibility.',
                  icon: <TrophyOutlined style={{ color: '#722ed1' }} />,
                },
              ]}
              renderItem={item => (
                <List.Item>
                  <List.Item.Meta
                    avatar={<Avatar icon={item.icon} />}
                    title={item.title}
                    description={item.description}
                  />
                </List.Item>
              )}
            />
          </Card>
        </TabPane>
      </Tabs>

      {/* Create Daycare Modal */}
      <Modal
        title="Create New Daycare"
        visible={createDaycareModalVisible}
        onOk={handleCreateDaycare}
        onCancel={() => setCreateDaycareModalVisible(false)}
        confirmLoading={actionLoading}
      >
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Daycare Name:</label>
          <Input
            placeholder="Enter daycare name"
            value={daycareName}
            onChange={(e) => setDaycareName(e.target.value)}
            maxLength={32}
          />
        </div>
        
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Description:</label>
          <TextArea
            placeholder="Enter daycare description"
            value={daycareDescription}
            onChange={(e) => setDaycareDescription(e.target.value)}
            maxLength={256}
            rows={4}
          />
        </div>
        
        <div>
          <label style={{ display: 'block', marginBottom: 8 }}>Fee Per Block:</label>
          <InputNumber
            style={{ width: '100%' }}
            min={1}
            max={100}
            value={feePerBlock}
            onChange={setFeePerBlock}
            addonAfter="BITS"
          />
          <p style={{ marginTop: 8, color: '#8c8c8c' }}>
            <DollarOutlined /> This is the fee pet owners will pay per block for listing their pets in your daycare.
          </p>
        </div>
      </Modal>

      {/* Create Listing Modal */}
      <Modal
        title="Create New Listing"
        visible={createListingModalVisible}
        onOk={handleCreateListing}
        onCancel={() => setCreateListingModalVisible(false)}
        confirmLoading={actionLoading}
      >
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Select Daycare:</label>
          <Select
            style={{ width: '100%' }}
            placeholder="Select a daycare"
            value={selectedDaycare}
            onChange={setSelectedDaycare}
          >
            {[...myDaycares, ...availableDaycares].map(daycare => (
              <Option key={daycare.id} value={daycare.id}>
                {daycare.name} - {daycare.fee_per_block} BITS per block
              </Option>
            ))}
          </Select>
        </div>
        
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
        
        <div>
          <label style={{ display: 'block', marginBottom: 8 }}>Duration (blocks):</label>
          <InputNumber
            style={{ width: '100%' }}
            min={100}
            max={10000}
            value={listingDuration}
            onChange={setListingDuration}
          />
          <p style={{ marginTop: 8, color: '#8c8c8c' }}>
            <DollarOutlined /> Total Fee: {selectedDaycare && feePerBlock * listingDuration} BITS
          </p>
        </div>
      </Modal>

      {/* Perform Care Action Modal */}
      <Modal
        title="Perform Care Action"
        visible={performCareModalVisible}
        onOk={handlePerformCareAction}
        onCancel={() => setPerformCareModalVisible(false)}
        confirmLoading={actionLoading}
      >
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>Care Action:</label>
          <Select
            style={{ width: '100%' }}
            value={selectedCareAction}
            onChange={setSelectedCareAction}
          >
            {careActions.map(action => (
              <Option key={action.value} value={action.value}>
                {action.icon} {action.label}
              </Option>
            ))}
          </Select>
        </div>
        
        {selectedCareAction === 'Socialize' && (
          <div>
            <label style={{ display: 'block', marginBottom: 8 }}>Target Pet:</label>
            <Select
              style={{ width: '100%' }}
              placeholder="Select a target pet"
              value={selectedTargetPet}
              onChange={setSelectedTargetPet}
            >
              {pets.map(pet => (
                <Option key={pet.id} value={pet.id}>{pet.name}</Option>
              ))}
            </Select>
            <p style={{ marginTop: 8, color: '#8c8c8c' }}>
              <TeamOutlined /> Socializing requires another pet to interact with.
            </p>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default DaycarePanel;