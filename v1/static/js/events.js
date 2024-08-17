async function loadGpus() {
    const response = await fetch('/gpus');
    const gpus = await response.json();
    const gpusDiv = document.getElementById('gpus');
    gpusDiv.innerHTML = ''; 

    gpus.forEach(gpu => {
        addGpuToDOM(gpu);
    });
}

function addGpuToDOM(gpu) {
    const gpusDiv = document.getElementById('gpus');

    const gpuDiv = document.createElement('div');
    gpuDiv.className = 'gpu-item';
    gpuDiv.textContent = `${gpu.name} | Temp: ${gpu.temp}°C | Watt: ${gpu.watt}W`;
    gpuDiv.setAttribute('data-id', gpu.id);  // Set the data-id attribute

    // Create the delete button
    const deleteButton = document.createElement('button');
    deleteButton.textContent = 'Delete';
    deleteButton.addEventListener('click', async () => {
        const confirmed = confirm(`Are you sure you want to delete ${gpu.name}?`);
        if (confirmed) {
            const gpuId = gpuDiv.getAttribute('data-id');  // Get the ID from the data-id attribute
            console.log(`Attempting to delete GPU with ID: ${gpuId}`); // Debug log
            try {
                const response = await fetch(`/gpus/${gpuId}`, {
                    method: 'DELETE'
                });
                
                if (response.ok) {
                    console.log(`GPU with ID: ${gpuId} deleted successfully`);
                    loadGpus(); // Refresh the list after deletion
                } else {
                    console.error('Failed to delete GPU:', await response.text());
                }
            } catch (error) {
                console.error('Error during GPU deletion:', error);
            }
        }
    });

    gpuDiv.appendChild(deleteButton);
    gpusDiv.appendChild(gpuDiv);
}

document.getElementById('createGpu').addEventListener('click', async () => {
    const gpuName = prompt('Enter GPU name:');
    const gpuTemp = prompt('Enter GPU temperature:');
    const gpuWatt = prompt('Enter GPU wattage:');

    if (gpuName && gpuTemp && gpuWatt) {
        const response = await fetch('/gpus', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: gpuName, temp: parseFloat(gpuTemp), watt: parseFloat(gpuWatt) })
        });

        if (response.ok) {
            const newGpu = await response.json();
            addGpuToDOM(newGpu);
        } else {
            console.error('Failed to add GPU:', await response.text());
        }
    }
});

document.addEventListener('DOMContentLoaded', loadGpus);