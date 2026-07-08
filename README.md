# Fossim (backend)

Rust with Axum backend for the [Fossim application](https://github.com/nekename/fossim-application), providing community-specific forge (e.g. GitHub) authentication credentials, event notifications, and other services to Fossim clients

## Backend setup

1. Set up your installation environment
	1. Either install Docker/Podman and Docker/Podman Compose and create a directory containing the `compose.yaml` file from this repository
	2. or install Rust and clone this repository
2. Create a GitHub App from GitHub Developer Settings (other forge providers may be supported in the future)
	- Set the app name to `[username]'s Fossim` or `[project name]'s Fossim` (or something else if you prefer)
	- Enable the Device Flow for client authentication
	- Enable a webhook for the app pointing to `https://fossim.yourdomain.com/api/webhook/github` (replace `yourdomain.com` with your domain)
		- Use a high-entropy string for the webhook secret (e.g. `openssl rand -hex 32`)
	- Enable the following repository permissions for your app:
		- `Discussions`: `Read and write`
	- Subscribe to the following webhook events for your app:
		- `Discussion`
		- `Discussion comment`
	- Generate a private key and client secret for your app
3. Install your GitHub App on the repositories you want to use with Fossim (or install it on your entire organization)
4. Fill out the placeholder values in your `compose.yaml` file (or create a `.env` file in the root of the repository) with the following content:

```
OAUTH_GITHUB_CLIENT_ID=<your_github_client_id>
OAUTH_GITHUB_CLIENT_SECRET=<your_github_client_secret>

WEBHOOK_GITHUB_SECRET=<your_github_webhook_secret>
```

5. Run the backend with `sudo docker compose up`, `podman-compose up` or `PORT=57216 cargo run` and point `https://fossim.yourdomain.com` to `http://localhost:57216` in your reverse proxy configuration (e.g. Nginx, Caddy, etc.) or using a Cloudflare Tunnel

## Per-repository setup

1. Enable GitHub Discussions in each repository's settings
2. Create a `Channels` discussion category in each repository
	- It is strongly recommended to use the `Announcement` category type for your `Channels` category, as this will allow you to create a curated list of channels for your community members that anyone can message in, while not allowing anyone to create new channels
	- Even if you use the `Announcement` category type, anyone will be able to create new threads (n.b.: not channels) by using other discussion categories
3. Add a `.fossim.json` file in the root of the repositories of your open-source projects with the following content, replacing `yourdomain.com` with your domain:

```json
{
	"host": "https://fossim.yourdomain.com",
	"name": "Your Project Name (optional)",
	"icon": "https://yourdomain.com/path/to/project/icon.png (optional)",
	"banner": "https://yourdomain.com/path/to/project/banner.png (optional)"
}
```
