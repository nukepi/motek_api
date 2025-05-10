import { executeCommand, printHeader } from './utils.js';
import { join } from 'path';
import spawn from 'cross-spawn';
import chalk from 'chalk';

async function dev() {
  printHeader('Running Motek UI in development mode');
  
  console.log(chalk.yellow('Note: This script runs the app in development mode.'));
  console.log(chalk.yellow('Press Ctrl+C to exit.\n'));

  // Run Flutter app
  const flutterProcess = spawn('flutter', ['run', '--hot'], { stdio: 'inherit' });

  // Handle process termination
  flutterProcess.on('close', (code) => {
    if (code !== 0 && code !== null) {
      console.error(chalk.red(`\nFlutter process exited with code ${code}`));
    }
    process.exit(code || 0);
  });

  // Handle termination signals
  process.on('SIGINT', () => {
    console.log(chalk.yellow('\nStopping development mode...'));
    flutterProcess.kill('SIGINT');
  });
}

dev().catch(err => {
  console.error('\nError while running development mode:', err);
  process.exit(1);
});
