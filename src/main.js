const { invoke } = window.__TAURI__.tauri;


function closeApp()
{
  invoke('close_window')
}


window.addEventListener('DOMContentLoaded', () => {
  const quitButton = document.getElementById('close-btn');
  if (quitButton) {
    quitButton.addEventListener('click', () => {
      closeApp();
    });
  }
});