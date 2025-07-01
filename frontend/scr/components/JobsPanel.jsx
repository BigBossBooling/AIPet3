import React, { useState, useEffect } from 'react';
import { Card, Button, Select, Table, Tag, Modal, InputNumber, Spin, notification, Tabs, Statistic, Row, Col, Progress, Alert } from 'antd';
import { 
  DollarOutlined, 
  ClockCircleOutlined, 
  CheckCircleOutlined,
  CloseCircleOutlined,
  PlusOutlined,
  TrophyOutlined,
  BulbOutlined,
  FireOutlined,
  ReloadOutlined
} from '@ant-design/icons';
import critterCraftAPI, { ConnectionError, TransactionError, QueryError } from '../crittercraft_api_improved';

const { Option } = Select;
const { TabPane } = Tabs;

/**
 * JobsPanel component for starting and managing jobs
 */
const JobsPanel = ({ pets = [] }) => {
  const [loading, setLoading] = useState(true);
  const [activeJobs, setActiveJobs] = useState([]);
  const [completedJobs, setCompletedJobs] = useState([]);
  const [startJobModalVisible, setStartJobModalVisible] = useState(false);
  const [selectedPet, setSelectedPet] = useState(null);
  const [selectedJobType, setSelectedJobType] = useState('CrystalMining');
  const [duration, setDuration] = useState(500);
  const [actionLoading, setActionLoading] = useState(false);
  const [refreshing, setRefreshing] = useState(false);

  // Job type options
  const jobTypes = [
    { value: 'CrystalMining', label: 'Crystal Mining', stat: 'Strength', description: 'Mine crystals in the caves' },
    { value: 'BioluminescentGuide', label: 'Bioluminescent Guide', stat: 'Charisma', description: 'Guide visitors through bioluminescent areas' },
    { value: 'HerbalistAssistant', label: 'Herbalist Assistant', stat: 'Intelligence', description: 'Assist the herbalist in gathering and processing herbs' },
  ];

  /**
   * Initialize component, connect to blockchain and fetch initial data
   */
  useEffect(() => {
    // Initialize connection and fetch jobs
    const initializeComponent = async () => {
      try {
        // Try to connect if not already connected
        if (!critterCraftAPI.isConnected) {
          await critterCraftAPI.connect();
        }
        
        // Update connection status
        setConnectionStatus({
          connected: critterCraftAPI.isConnected,
          checking: false
        });
        
        // Fetch jobs
        fetchJobs();
      } catch (error) {
        setConnectionStatus({
          connected: false,
          checking: false
        });
        
        // Use our centralized error handling
        handleError(error, 'initialization');
        
        // Still set loading to false
        setLoading(false);
      }
    };
    
    initializeComponent();
  }, []);

  /**
   * Fetch active and completed jobs
   * @returns {Promise<void>}
   */
  const fetchJobs = async () => {
    try {
      setRefreshing(true);
      
      // Fetch active jobs
      const activeJobIds = await critterCraftAPI.getActiveJobsByOwner();
      
      // Fetch details for each active job
      const activeJobsPromises = activeJobIds.map(id => critterCraftAPI.getJob(id));
      const activeJobsData = await Promise.all(activeJobsPromises);
      
      // Fetch completed jobs using our new API method
      const completedJobsData = await critterCraftAPI.getCompletedJobsByOwner(null, 10);
      
      // Update state with fetched data
      setActiveJobs(activeJobsData);
      setCompletedJobs(completedJobsData);
    } catch (error) {
      // Use our centralized error handling
      handleError(error, 'fetching jobs');
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  };
  
  /**
   * Retry connection to the blockchain
   * @returns {Promise<void>}
   */
  const retryConnection = async () => {
    notification.info({
      message: 'Reconnecting',
      description: 'Attempting to reconnect to the blockchain...',
    });
    
    try {
      setConnectionStatus({ ...connectionStatus, checking: true });
      await critterCraftAPI.connect();
      
      setConnectionStatus({ 
        connected: critterCraftAPI.isConnected, 
        checking: false 
      });
      
      notification.success({
        message: 'Connected',
        description: 'Successfully reconnected to the blockchain.',
      });
      
      // Refresh data
      fetchJobs();
    } catch (error) {
      setConnectionStatus({ connected: false, checking: false });
      handleError(error, 'reconnection');
    }
  };
  
  // Set up automatic reconnection attempts
  useEffect(() => {
    let reconnectInterval = null;
    
    // If not connected, try to reconnect every 30 seconds
    if (!connectionStatus.connected && !connectionStatus.checking) {
      reconnectInterval = setInterval(() => {
        console.log('Attempting automatic reconnection...');
        retryConnection();
      }, 30000); // Try every 30 seconds
    }
    
    // Clean up interval on unmount or when connection status changes
    return () => {
      if (reconnectInterval) {
        clearInterval(reconnectInterval);
      }
    };
  }, [connectionStatus.connected, connectionStatus.checking]);
  
  // Set up periodic job data refresh
  useEffect(() => {
    let refreshInterval = null;
    
    // Only set up refresh if connected
    if (connectionStatus.connected) {
      refreshInterval = setInterval(() => {
        console.log('Auto-refreshing job data...');
        // Don't show loading indicator for auto-refresh
        fetchJobs();
      }, 60000); // Refresh every minute
    }
    
    // Clean up interval on unmount or when connection status changes
    return () => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
    };
  }, [connectionStatus.connected]);

  /**
   * Start a new job
   * @returns {Promise<void>}
   */
  const handleStartJob = async () => {
    if (!selectedPet) {
      notification.warning({
        message: 'No pet selected',
        description: 'Please select a pet for the job.',
      });
      return;
    }

    try {
      setActionLoading(true);
      
      // Convert string values to enum indices
      const jobTypeIndex = jobTypes.findIndex(type => type.value === selectedJobType);
      
      // Get the selected job type for the success message
      const selectedJobTypeInfo = jobTypes.find(type => type.value === selectedJobType);
      
      await critterCraftAPI.startJob(
        selectedPet,
        jobTypeIndex,
        duration
      );
      
      notification.success({
        message: 'Job Started',
        description: `Your pet has started a ${selectedJobTypeInfo.label} job!`,
      });
      
      setStartJobModalVisible(false);
      fetchJobs();
    } catch (error) {
      handleError(error, 'starting job');
      
      // Add a reconnect button if it's a connection error
      if (error instanceof ConnectionError) {
        notification.info({
          message: 'Reconnection Available',
          description: 'You can try to reconnect to the blockchain.',
          btn: <Button type="primary" onClick={retryConnection}>Reconnect</Button>,
          duration: 10,
        });
      }
    } finally {
      setActionLoading(false);
    }
  };

  /**
   * Complete a job
   * @param {string|number} jobId - ID of the job to complete
   * @returns {Promise<void>}
   */
  const handleCompleteJob = async (jobId) => {
    try {
      setActionLoading(true);
      
      await critterCraftAPI.completeJob(jobId);
      
      notification.success({
        message: 'Job Completed',
        description: 'The job has been completed successfully!',
        icon: <CheckCircleOutlined style={{ color: '#52c41a' }} />,
      });
      
      fetchJobs();
    } catch (error) {
      handleError(error, 'completing job');
      
      // Add a reconnect button if it's a connection error
      if (error instanceof ConnectionError) {
        notification.info({
          message: 'Reconnection Available',
          description: 'You can try to reconnect to the blockchain.',
          btn: <Button type="primary" onClick={retryConnection}>Reconnect</Button>,
          duration: 10,
        });
      }
    } finally {
      setActionLoading(false);
    }
  };

  /**
   * Cancel a job
   * @param {string|number} jobId - ID of the job to cancel
   * @returns {Promise<void>}
   */
  const handleCancelJob = async (jobId) => {
    try {
      setActionLoading(true);
      
      await critterCraftAPI.cancelJob(jobId);
      
      notification.success({
        message: 'Job Canceled',
        description: 'The job has been canceled successfully.',
        icon: <CloseCircleOutlined style={{ color: '#ff4d4f' }} />,
      });
      
      fetchJobs();
    } catch (error) {
      handleError(error, 'canceling job');
      
      // Add a reconnect button if it's a connection error
      if (error instanceof ConnectionError) {
        notification.info({
          message: 'Reconnection Available',
          description: 'You can try to reconnect to the blockchain.',
          btn: <Button type="primary" onClick={retryConnection}>Reconnect</Button>,
          duration: 10,
        });
      }
    } finally {
      setActionLoading(false);
    }
  };

  /**
   * Calculate time remaining for a job
   * @param {Object} job - Job object
   * @returns {string} - Formatted time remaining string
   */
  const calculateTimeRemaining = (job) => {
    if (!job || !job.ends_at) return 'Unknown';
    
    try {
      const now = Date.now();
      const endsAt = new Date(job.ends_at).getTime();
      
      // If job is already finished
      if (now >= endsAt) return 'Finished';
      
      // Calculate remaining time in milliseconds
      const remainingMs = endsAt - now;
      
      // Convert to hours, minutes, seconds
      const hours = Math.floor(remainingMs / (1000 * 60 * 60));
      const minutes = Math.floor((remainingMs % (1000 * 60 * 60)) / (1000 * 60));
      const seconds = Math.floor((remainingMs % (1000 * 60)) / 1000);
      
      // Format the time remaining
      if (hours > 0) {
        return `${hours}h ${minutes}m`;
      } else if (minutes > 0) {
        return `${minutes}m ${seconds}s`;
      } else {
        return `${seconds}s`;
      }
    } catch (error) {
      console.error('Error calculating time remaining:', error);
      return 'Error';
    }
  };
  
  /**
   * Calculate job progress as a percentage
   * @param {Object} job - Job object
   * @returns {number} - Progress percentage (0-100)
   */
  const calculateJobProgress = (job) => {
    if (!job || !job.started_at || !job.ends_at) return 0;
    
    const currentTime = Date.now(); // Current time in milliseconds
    const startedAt = new Date(job.started_at).getTime();
    const endsAt = new Date(job.ends_at).getTime();
    
    // Handle edge cases
    if (startedAt >= endsAt) return 0;
    
    const totalDuration = endsAt - startedAt;
    const elapsed = currentTime - startedAt;
    
    // Ensure progress is between 0 and 100
    return Math.min(Math.max(Math.floor((elapsed / totalDuration) * 100), 0), 100);
  };
  
  /**
   * Format date for display
   * @param {string} dateString - Date string to format
   * @returns {string} - Formatted date string
   */
  const formatDate = (dateString) => {
    try {
      if (!dateString) return 'N/A';
      
      const date = new Date(dateString);
      
      // Check if date is valid
      if (isNaN(date.getTime())) return 'Invalid date';
      
      // Format date as: "Jan 1, 2023, 12:00 PM"
      return date.toLocaleString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      });
    } catch (error) {
      console.error('Error formatting date:', error);
      return 'Error';
    }
  };
  
  /**
   * Get pet name from pet ID
   * @param {string|number} petId - Pet ID
   * @returns {string} - Pet name or ID if not found
   */
  const getPetName = (petId) => {
    if (!petId) return 'Unknown';
    
    const pet = pets.find(p => p.id === petId || p.id === Number(petId));
    return pet ? pet.name : `Pet #${petId}`;
  };
  
  /**
   * Get job type label from job type value
   * @param {string} jobType - Job type value
   * @returns {string} - Job type label
   */
  const getJobTypeLabel = (jobType) => {
    if (!jobType) return 'Unknown';
    
    const jobTypeInfo = jobTypes.find(type => type.value === jobType);
    return jobTypeInfo ? jobTypeInfo.label : jobType;
  };
  
  /**
   * Format reward values for display
   * @param {string|number} value - Reward value
   * @param {string} type - Type of reward ('experience', 'currency', or 'bonus')
   * @returns {string} - Formatted reward string
   */
  const formatReward = (value, type) => {
    if (value === null || value === undefined) return 'None';
    
    // Convert to number if it's a string
    const numValue = typeof value === 'string' ? parseInt(value, 10) : value;
    
    // Return formatted value based on type
    switch (type) {
      case 'experience':
        return `${numValue} XP`;
      case 'currency':
        return `${numValue} CC`;
      case 'bonus':
        return `+${numValue}`;
      default:
        return String(numValue);
    }
  };

  /**
   * Check if a job is complete (either by status or time)
   * @param {Object} job - Job object to check
   * @returns {boolean} - True if job is complete, false otherwise
   */
  const isJobComplete = (job) => {
    if (!job) return false;
    
    try {
      // If the job has a status field and it's "Completed", it's already completed
      if (job.status && job.status === "Completed") {
        return true;
      }
      
      // If the job is canceled, it's not considered complete
      if (job.status && job.status === "Canceled") {
        return false;
      }
      
      // Otherwise check if the current time is past the end time
      if (!job.ends_at) return false;
      
      const currentTime = Date.now();
      const endsAt = new Date(job.ends_at).getTime();
      
      return currentTime >= endsAt;
    } catch (error) {
      console.error('Error checking if job is complete:', error);
      return false; // Default to false on error
    }
  };
  
  /**
   * Check if a job can be completed (finished but not yet claimed)
   * @param {Object} job - Job object to check
   * @returns {boolean} - True if job can be completed, false otherwise
   */
  const canCompleteJob = (job) => {
    if (!job) return false;
    
    try {
      // If the job is already completed, it can't be completed again
      if (job.status && job.status === "Completed") {
        return false;
      }
      
      // If the job is canceled, it can't be completed
      if (job.status && job.status === "Canceled") {
        return false;
      }
      
      // Check if the job has a valid end time
      if (!job.ends_at) return false;
      
      // Check if the current time is past the end time
      const currentTime = Date.now();
      const endsAt = new Date(job.ends_at).getTime();
      
      return currentTime >= endsAt;
    } catch (error) {
      console.error('Error checking if job can be completed:', error);
      return false; // Default to false on error
    }
  };
  
  // Column definitions for active jobs table
  const activeJobColumns = [
    {
      title: 'Status',
      key: 'status',
      render: (_, record) => {
        const canComplete = canCompleteJob(record);
        const isFinished = isJobComplete(record);
        
        if (record.status === "Canceled") {
          return <Tag color="red">Canceled</Tag>;
        } else if (record.status === "Completed") {
          return <Tag color="green">Completed</Tag>;
        } else if (isFinished) {
          return <Tag color="gold">Ready to Complete</Tag>;
        } else {
          return <Tag color="blue">In Progress</Tag>;
        }
      },
    },
    {
    // If the job is canceled, it can't be completed
    if (job.status && job.status === "Canceled") {
      return false;
    }
    
    // Check if the current time is past the end time
    const currentTime = Date.now();
    const endsAt = new Date(job.ends_at).getTime();
    
    return currentTime >= endsAt;
  };

  // Columns for the active jobs table
  const activeJobsColumns = [
    {
      title: 'Job',
      dataIndex: 'job_type',
      key: 'job_type',
      render: (jobType) => {
        const canjob = jobTypes.find(type => type.value === jobType);
        return job ? job.label : jobType;
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
              title={!canComplete ? "Job not yet finished" : !connectionStatus.connected ? "Not connected to blockchain" : "Complete this job"}
    },
    {
      title: 'Started',
      dataIndex: 'started_at',
      key: 'started_at',
      render: (startedAt) => new Date(startedAt).toLocaleString(),
    },
    {
      title: 'Ends At',
      dataIndex: 'ends_at',
      key: 'ends_at',
      render: (endsAt) => new Date(endsAt).toLocaleString(),
    },
    {
      title: 'Status',
      key: 'status',
      render: (_, record) => {
        const canComplete = canCompleteJob(record);
        const isFinished = isJobComplete(record);
        
        if (record.status === "Canceled") {
          return <Tag color="red">Canceled</Tag>;
        } else if (record.status === "Completed") {
          return <Tag color="green">Completed</Tag>;
        } else if (isFinished) {
          return <Tag color="gold">Ready to Complete</Tag>;
        } else {
          return <Tag color="blue">In Progress</Tag>;
        }
      },
    },
    {
      title: 'Progress',
      key: 'progress',
      render: (_, record) => {
        const progress = calculateJobProgress(record);
        const isComplete = isJobComplete(record);
        
        return (
          <Progress 
            percent={progress} 
            status={isComplete ? 'success' : 'active'} 
            size="small" 
          />
        );
      },
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => {
        const canComplete = canCompleteJob(record);
        const isFinished = isJobComplete(record);
        
        return (
          <div>
            <Button 
              type="primary" 
              icon={<CheckCircleOutlined />} 
              onClick={() => handleCompleteJob(record.id)}
              disabled={!canComplete || !connectionStatus.connected || !walletStatus.connected}
              style={{ marginRight: 8 }}
              title={!canComplete 
                ? "Job not yet finished" 
                : !connectionStatus.connected 
                ? "Not connected to blockchain" 
                : !walletStatus.connected 
                ? "Connect your wallet first" 
                : "Complete this job"}
            >
              Complete
            </Button>
            <Button 
              danger 
              icon={<CloseCircleOutlined />} 
              onClick={() => handleCancelJob(record.id)}
              disabled={isFinished || !connectionStatus.connected || !walletStatus.connected}
              title={isFinished 
                ? "Cannot cancel completed job" 
                : !connectionStatus.connected 
                ? "Not connected to blockchain" 
                : !walletStatus.connected 
                ? "Connect your wallet first" 
                : "Cancel this job"}
            >
              Cancel
            </Button>
          </div>
        );
      },
    },
  ];

  // Columns for the completed jobs table
  const completedJobsColumns = [
    {
      title: 'Job',
      dataIndex: 'job_type',
      key: 'job_type',
      render: (jobType) => {
        const job = jobTypes.find(type => type.value === jobType);
        return job ? job.label : jobType;
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
      title: 'Duration',
      dataIndex: 'duration',
      key: 'duration',
      render: (duration) => `${duration} blocks`,
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
      title: 'Bonus Reward',
      dataIndex: 'bonus_reward',
      key: 'bonus_reward',
      render: (bonus) => bonus ? <Tag color="green">{bonus} BITS</Tag> : 'None',
    },
    {
      title: 'Completed',
      dataIndex: 'completed_at',
      key: 'completed_at',
      render: (completedAt) => new Date(completedAt).toLocaleString(),
    },
  ];

  // State for connection status
  const [connectionStatus, setConnectionStatus] = useState({
    connected: critterCraftAPI.isConnected,
    checking: false
  });
  
  // State for wallet connection
  const [walletStatus, setWalletStatus] = useState({
    connected: false,
    accounts: [],
    loading: false
  });

  // Check connection status
  const checkConnection = async () => {
    setConnectionStatus({ ...connectionStatus, checking: true });
    try {
      const connected = await critterCraftAPI.connect();
      setConnectionStatus({ connected, checking: false });
      if (connected) {
        fetchJobs();
      }
    } catch (error) {
      setConnectionStatus({ connected: false, checking: false });
      notification.error({
        message: 'Connection Failed',
        description: error.message,
      });
    }
  };

  // Initialize connection status on mount
  useEffect(() => {
    checkConnection();
  }, []);
  
  /**
   * Handle errors in a consistent way
   * @param {Error} error - The error to handle
   * @param {string} context - Context where the error occurred
   */
  const handleError = (error, context) => {
    console.error(`Error in ${context}:`, error);
    
    // Handle different error types
    if (error instanceof ConnectionError) {
      notification.error({
        message: 'Connection Error',
        description: `Failed to connect to the blockchain: ${error.message}`,
        duration: 0, // Keep notification until manually closed
      });
    } else if (error instanceof WalletError) {
      notification.error({
        message: 'Wallet Error',
        description: error.message,
      });
    } else if (error instanceof TransactionError) {
      notification.error({
        message: 'Transaction Failed',
        description: error.message,
      });
    } else if (error instanceof QueryError) {
      notification.error({
        message: 'Data Fetch Error',
        description: `Failed to fetch data: ${error.message}`,
      });
    } else {
      notification.error({
        message: `Error in ${context}`,
        description: error.message || 'An unknown error occurred',
      });
    }
  };
  
  /**
   * Connect to wallet
   * @returns {Promise<void>}
   */
  const connectWallet = async () => {
    try {
      setWalletStatus({ ...walletStatus, loading: true });
      
      // Connect to wallet
      const accounts = await critterCraftAPI.connectWallet();
      
      // Set the first account as active if available
      if (accounts.length > 0) {
        critterCraftAPI.setAccount(accounts[0]);
      }
      
      setWalletStatus({
        connected: accounts.length > 0,
        accounts: accounts,
        loading: false
      });
      
      // Show success notification
      notification.success({
        message: 'Wallet Connected',
        description: `Connected to wallet with ${accounts.length} accounts.`,
      });
      
      // Refresh jobs with the new account
      fetchJobs();
    } catch (error) {
      setWalletStatus({
        connected: false,
        accounts: [],
        loading: false
      });
      
      handleError(error, 'wallet connection');
    }
  };

  return (
    <div>
      {/* Connection Status Alerts */}
      {!connectionStatus.connected && (
        <Alert
          message="Blockchain Connection Error"
          description="Not connected to the blockchain. Some features may not work properly."
          type="error"
          showIcon
          action={
            <Button 
              size="small" 
              type="primary" 
              loading={connectionStatus.checking}
              onClick={checkConnection}
            >
              Reconnect
            </Button>
          }
          style={{ marginBottom: 16 }}
        />
      )}
      
      {/* Wallet Connection Alert */}
      {connectionStatus.connected && !walletStatus.connected && (
        <Alert
          message="Wallet Not Connected"
          description="Connect your wallet to start and manage jobs."
          type="warning"
          showIcon
          action={
            <Button 
              size="small" 
              type="primary" 
              loading={walletStatus.loading}
              onClick={connectWallet}
            >
              Connect Wallet
            </Button>
          }
          style={{ marginBottom: 16 }}
        />
      )}
      
      <Card 
        title="Jobs" 
        extra={
          <div style={{ display: 'flex', gap: '8px' }}>
            <Button
              icon={<ReloadOutlined />}
              onClick={fetchJobs}
              loading={refreshing}
              title="Refresh jobs"
            />
            <Button 
              type="primary" 
              icon={<PlusOutlined />} 
              onClick={() => setStartJobModalVisible(true)}
              disabled={!connectionStatus.connected || !walletStatus.connected}
              title={!connectionStatus.connected 
                ? "Not connected to blockchain" 
                : !walletStatus.connected 
                ? "Connect your wallet first" 
                : "Start a new job"}
            >
              Start New Job
            </Button>
          </div>
        }
        style={{ width: '100%', marginBottom: 16 }}
      >
        <Tabs defaultActiveKey="active">
          <TabPane tab="Active Jobs" key="active">
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading jobs...</p>
              </div>
            ) : (
              <Table 
                dataSource={activeJobs} 
                columns={activeJobsColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={false}
                locale={{ emptyText: connectionStatus.connected 
                  ? 'No active jobs. Start a new job to earn BITS!' 
                  : 'Cannot load jobs. Please check your connection.' 
                }}
              />
            )}
          </TabPane>
          <TabPane tab="Completed Jobs" key="completed">
            {loading ? (
              <div style={{ textAlign: 'center', padding: 24 }}>
                <Spin size="large" />
                <p style={{ marginTop: 16 }}>Loading jobs...</p>
              </div>
            ) : (
              <Table 
                dataSource={completedJobs} 
                columns={completedJobsColumns} 
                rowKey="id"
                loading={refreshing}
                pagination={{ pageSize: 5 }}
                locale={{ emptyText: connectionStatus.connected 
                  ? 'No completed jobs yet. Complete a job to see your rewards!' 
                  : 'Cannot load jobs. Please check your connection.'
                }}
              />
            )}
          </TabPane>
        </Tabs>
      </Card>

      {/* Job Types Information */}
      <Card title="Job Types" style={{ width: '100%', marginBottom: 16 }}>
        <Row gutter={[16, 16]}>
          <Col span={8}>
            <Card>
              <Statistic
                title="Crystal Mining"
                value="Strength"
                prefix={<TrophyOutlined style={{ color: '#722ed1' }} />}
                valueStyle={{ color: '#722ed1' }}
              />
              <p>Mine valuable crystals in the caves. Requires high Strength. Longer duration but higher rewards.</p>
            </Card>
          </Col>
          <Col span={8}>
            <Card>
              <Statistic
                title="Bioluminescent Guide"
                value="Charisma"
                prefix={<FireOutlined style={{ color: '#fa8c16' }} />}
                valueStyle={{ color: '#fa8c16' }}
              />
              <p>Guide visitors through bioluminescent areas. Requires high Charisma. Medium duration and rewards.</p>
            </Card>
          </Col>
          <Col span={8}>
            <Card>
              <Statistic
                title="Herbalist Assistant"
                value="Intelligence"
                prefix={<BulbOutlined style={{ color: '#1890ff' }} />}
                valueStyle={{ color: '#1890ff' }}
              />
              <p>Assist the herbalist in gathering and processing herbs. Requires high Intelligence. Shorter duration but requires specific pet traits.</p>
            </Card>
          </Col>
        </Row>
      </Card>

      {/* Start Job Modal */}
      <Modal
        title="Start New Job"
        visible={startJobModalVisible}
        onOk={handleStartJob}
        onCancel={() => setStartJobModalVisible(false)}
        confirmLoading={actionLoading}
        okButtonProps={{ 
          disabled: !connectionStatus.connected || !walletStatus.connected || !selectedPet 
        }}
      >
        {!connectionStatus.connected && (
          <Alert
            message="Connection Required"
            description="You need to be connected to the blockchain to start a job."
            type="warning"
            showIcon
            action={
              <Button 
                size="small" 
                type="primary" 
                loading={connectionStatus.checking}
                onClick={checkConnection}
              >
                Reconnect
              </Button>
            }
            style={{ marginBottom: 16 }}
          />
        )}
        
        {connectionStatus.connected && !walletStatus.connected && (
          <Alert
            message="Wallet Connection Required"
            description="You need to connect your wallet to start a job."
            type="warning"
            showIcon
            action={
              <Button 
                size="small" 
                type="primary" 
                loading={walletStatus.loading}
                onClick={connectWallet}
              >
                Connect Wallet
              </Button>
            }
            style={{ marginBottom: 16 }}
          />
        )}
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
          <label style={{ display: 'block', marginBottom: 8 }}>Job Type:</label>
          <Select
            style={{ width: '100%' }}
            value={selectedJobType}
            onChange={setSelectedJobType}
          >
            {jobTypes.map(type => (
              <Option key={type.value} value={type.value}>
                {type.label} - Requires {type.stat}
              </Option>
            ))}
          </Select>
          <p style={{ marginTop: 4, color: '#8c8c8c' }}>
            {jobTypes.find(type => type.value === selectedJobType)?.description}
          </p>
        </div>
        
        <div>
          <label style={{ display: 'block', marginBottom: 8 }}>Duration (blocks):</label>
          <InputNumber
            style={{ width: '100%' }}
            min={100}
            max={10000}
            value={duration}
            onChange={setDuration}
          />
          <p style={{ marginTop: 8, color: '#8c8c8c' }}>
            <ClockCircleOutlined /> Longer jobs provide better rewards but take more time to complete.
          </p>
          <p style={{ color: '#8c8c8c' }}>
            <DollarOutlined /> Rewards are based on job type, duration, and pet stats.
          </p>
        </div>
      </Modal>
    </div>
  );
};

export default JobsPanel;