import { Client, OAuth2, generateCodeVerifier, generateCodeChallenge, type OAuth2Config, type ClientConfig, type OAuth2Token } from '@xdevplatform/xdk';
import { createServer, type IncomingMessage, type ServerResponse } from 'http';
import { appendFile, writeFile } from 'fs/promises';
import { randomBytes } from 'crypto';
import { exec } from 'child_process';

function openBrowser(url: string): void {
  const command: string = process.platform === 'darwin' ? `open "${url}"` : process.platform === 'win32' ? `start "" "${url}"` : `xdg-open "${url}"`;

  exec(command, (error: Error | null): void => {
    if (error) {
      console.warn('Could not open browser automatically. Open this URL manually:');
      console.warn(url);
    }
  });
}

function getUrlFromRequest(req: IncomingMessage): URL {
  const host: string = req.headers.host ?? 'localhost:3001';
  const requestUrl: string = req.url ?? '/';
  return new URL(requestUrl, `http://${host}`);
}

async function waitForAuthCode(expectedState: string, redirectUri: string, timeoutMs: number = 180000): Promise<string> {
  const redirectUrl: URL = new URL(redirectUri);
  const port: number = Number(redirectUrl.port || 3001);
  const callbackPath: string = redirectUrl.pathname || '/';

  return new Promise<string>((resolve, reject): void => {
    const timeout = setTimeout((): void => {
      server.close();
      reject(new Error(`Timed out waiting for OAuth callback after ${timeoutMs / 1000} seconds.`));
    }, timeoutMs);

    const server = createServer((req: IncomingMessage, res: ServerResponse): void => {
      const url: URL = getUrlFromRequest(req);

      if (url.pathname !== callbackPath) {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('Not found');
        return;
      }

      const code: string | null = url.searchParams.get('code');
      const state: string | null = url.searchParams.get('state');
      const error: string | null = url.searchParams.get('error');

      if (error) {
        clearTimeout(timeout);
        res.writeHead(400, { 'Content-Type': 'text/plain' });
        res.end(`Authorization failed: ${error}. You can close this tab.`);
        server.close();
        reject(new Error(`OAuth provider returned error: ${error}`));
        return;
      }

      if (!code) {
        clearTimeout(timeout);
        res.writeHead(400, { 'Content-Type': 'text/plain' });
        res.end('Missing code query parameter. You can close this tab.');
        server.close();
        reject(new Error('Missing code query parameter in callback URL.'));
        return;
      }

      if (state !== expectedState) {
        clearTimeout(timeout);
        res.writeHead(400, { 'Content-Type': 'text/plain' });
        res.end('Invalid state value. You can close this tab.');
        server.close();
        reject(new Error('State mismatch in OAuth callback.'));
        return;
      }

      clearTimeout(timeout);
      res.writeHead(200, { 'Content-Type': 'text/plain' });
      res.end('Authorization complete. You can close this tab.');
      server.close((): void => resolve(code));
    });

    server.listen(port, '127.0.0.1', (): void => {
      console.log(`Waiting for OAuth callback on http://127.0.0.1:${port}${callbackPath}`);
    });
  });
}

async function persistTokens(tokens: OAuth2Token): Promise<void> {
  const serialized: string = JSON.stringify(tokens, null, 2);
  await writeFile('.oauth-tokens.json', `${serialized}\n`, 'utf8');
  await appendFile('.gitignore', '\n.oauth-tokens.json\n', 'utf8').catch((): void => {
    // Best effort only.
  });
}

(async (): Promise<void> => {
  const oauth2Config: OAuth2Config = {
    clientId: process.env.X_CLIENT_ID ?? 'QTZxdzNWRGJIdDZ5TkIwTm8tSUo6MTpjaQ',
    clientSecret: process.env.X_CLIENT_SECRET ?? 'M5muKqnVDiLb3LuIgjvgXn_Nd1cuDbXv33oxWz2Cv4mY_PkZ0r',
    redirectUri: process.env.X_REDIRECT_URI ?? 'http://localhost:3001/callback',
    scope: ['tweet.read', 'users.read', 'offline.access', 'users.email']
  };

  const oauth2: OAuth2 = new OAuth2(oauth2Config);

  const state: string = randomBytes(16).toString('hex');
  const codeVerifier: string = generateCodeVerifier();
  const codeChallenge: string = await generateCodeChallenge(codeVerifier);

  oauth2.setPkceParameters(codeVerifier, codeChallenge);

  const authUrl: string = await oauth2.getAuthorizationUrl(state);
  console.log('Open this URL to authorize:');
  console.log(authUrl);
  openBrowser(authUrl);

  const authCode: string = await waitForAuthCode(state, oauth2Config.redirectUri);

  const tokens: OAuth2Token = await oauth2.exchangeCode(authCode, codeVerifier);
  await persistTokens(tokens);
  console.log('OAuth token exchange succeeded. Tokens saved to .oauth-tokens.json');

  const config: ClientConfig = {
    accessToken: tokens.access_token
  };

  const client: Client = new Client(config);

  // Keep the created client available for follow-up API calls in this script.
  void client;
})().catch((error: unknown): void => {
  const message: string = error instanceof Error ? error.message : String(error);
  console.error(`OAuth flow failed: ${message}`);
  process.exitCode = 1;
});
