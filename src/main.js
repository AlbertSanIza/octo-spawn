const { invoke } = window.__TAURI__.core;

const spawnBtn = document.querySelector("#spawn-btn");
const testBtn = document.querySelector("#test-btn");
const status = document.querySelector("#status");

function setStatus(message, type = "") {
  status.textContent = message;
  status.className = type;
}

// Spawn a new GitHub Desktop window
spawnBtn.addEventListener("click", async () => {
  setStatus("Spawning GitHub Desktop...", "");
  
  try {
    // TODO: Replace with actual command to spawn GitHub Desktop
    const result = await invoke("execute_command", { 
      command: "echo 'Spawning GitHub Desktop...'"
    });
    
    setStatus("✓ GitHub Desktop spawned!", "success");
    
    setTimeout(() => setStatus(""), 2000);
  } catch (error) {
    console.error(error);
    setStatus(`✗ Error: ${error}`, "error");
  }
});

// Test command execution
testBtn.addEventListener("click", async () => {
  setStatus("Testing command...", "");
  
  try {
    const result = await invoke("execute_command", { 
      command: "date"
    });
    
    console.log("Command output:", result);
    setStatus("✓ Test successful! Check console.", "success");
    
    setTimeout(() => setStatus(""), 2000);
  } catch (error) {
    console.error(error);
    setStatus(`✗ Error: ${error}`, "error");
  }
});
