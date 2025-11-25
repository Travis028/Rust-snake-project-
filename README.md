# Rust Snake Game

A classic Snake game implemented in Rust using the crossterm library for terminal input handling.

## Prerequisites

- Docker (for local development and deployment)
- Git (for version control)
- A Render.com account (for deployment)

## Local Development

1. Clone the repository:
   ```bash
   git clone <your-repository-url>
   cd rustysnake-project
   ```

2. Build and run with Docker:
   ```bash
   cd rustysnake-game
   docker build -t rustysnake .
   docker run -it --rm rustysnake
   ```

## Deployment to Render

### Prerequisites
- A GitHub/GitLab account with the repository pushed
- A Render.com account

### Steps

1. **Push your code** to a GitHub or GitLab repository

2. **Sign in to Render**
   - Go to [https://dashboard.render.com/](https://dashboard.render.com/)
   - Sign up or log in

3. **Create a new Web Service**
   - Click "New" and select "Web Service"
   - Connect your GitHub/GitLab account if not already connected
   - Select your repository

4. **Configure the service**
   - Name: `rustysnake-game` (or your preferred name)
   - Region: Choose the one closest to your users
   - Branch: `main` (or your default branch)
   - Build Command: `.` (dot)
   - Start Command: (leave empty, using Dockerfile's ENTRYPOINT)
   - Plan: Select "Free" for testing or "Standard" for production

5. **Deploy**
   - Click "Create Web Service"
   - Render will automatically build and deploy your application
   - The deployment logs will be visible in the dashboard

6. **Access your game**
   - Once deployed, you'll get a URL like `https://rustysnake-xxxx.onrender.com`
   - The game will be accessible via web terminal

## Game Controls
- Use arrow keys to control the snake
- Eat the food (`*`) to grow
- Avoid hitting the walls or yourself
- Press `q` to quit the game

## Troubleshooting
- If the build fails, check the logs in the Render dashboard
- Ensure all required files are included in your repository
- Check that the Dockerfile and render.yaml are in the correct locations

## License
[MIT](LICENSE)
