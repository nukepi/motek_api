import { executeCommand, withSpinner, printHeader } from './utils.js';

async function generate() {
  printHeader('Generating code with flutter_rust_bridge');

  await withSpinner('Generating code', async () => {
    await executeCommand('flutter_rust_bridge_codegen', ['generate']);
  });

  console.log('\nCode generated successfully! 🎉');
}

generate().catch(err => {
  console.error('\nError while generating code:', err);
  process.exit(1);
});
