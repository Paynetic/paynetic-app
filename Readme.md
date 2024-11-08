<h2 align='center'>Paynetic App</h2>

<p align='center'>Web frontend, backend, and smart contracts for https://paynetic.net</p>

<br>

#### Prerequisites

- Install [PNPM](https://pnpm.io/)

#### Setup

```bash
# Clone monorepo
git clone git@github.com:PayneticApp/paynetic-app

# Install packages
pnpm i
```

#### Run

** Launch stack or parts of it in a local k8s cluster:**

```bash
# Build backend apps for aarch64
$ npm run dev:build-aarch
# Start paynetic-api (Paynetic API + Postgres)
$ npm run skaffold:basic
```

```bash
# Run paynetic site in development mode
npm run prod:web:run

# Run paynetic web admin
npm run prod:web-admin:run
```

**Build**

```bash
# Build paynetic web app for production
npm run prod:web:build

# Build paynetic web admin
npm run prod:web-admin:build
```
