# CritterCraft Universe

CritterCraft Universe is an advanced AI-powered blockchain pet simulation platform that creates meaningful digital companions with evolving personalities.

## Key Features

- **Intelligent Companions**: AI-powered pets that develop unique personalities
- **Dynamic NFTs**: Continuously evolving digital assets that reflect real experiences
- **Custom Blockchain**: Purpose-built infrastructure optimized for pet simulation mechanics
- **Emotional Engagement**: Focus on meaningful relationships rather than purely transactional gameplay

## Technology Stack

### Backend
- Python 3.11
- Flask/FastAPI
- SQLAlchemy
- TensorFlow/PyTorch
- LangChain
- Substrate/Polkadot

### Frontend
- React 18
- TypeScript
- Vite
- Three.js
- Polkadot.js API
- Material UI

### Infrastructure
- Docker
- PostgreSQL
- Redis
- Nginx
- GitHub Actions

## Getting Started

### Prerequisites
- Docker and Docker Compose
- Node.js 18+
- Python 3.11+
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/your-org/crittercraft-universe.git
cd crittercraft-universe
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Start the application with Docker:
```bash
docker-compose up -d
```

4. Or run the services individually:

**Backend:**
```bash
cd blockchain_core/critter-craft
pip install -r requirements.txt
python main.py
```

**Frontend:**
```bash
cd frontend
npm install
npm run start
```

## Development

### Backend Development
```bash
# Run tests
cd blockchain_core/critter-craft
pytest

# Run with hot reload
uvicorn main:app --reload
```

### Frontend Development
```bash
cd frontend
npm run dev

# Run tests
npm run test
```

## Deployment

The application can be deployed using Docker Compose:

```bash
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Substrate and Polkadot teams for blockchain infrastructure
- OpenAI for AI capabilities
- The open-source community for various libraries and tools