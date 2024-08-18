document.addEventListener('DOMContentLoaded', loadFarms);

async function loadFarms() {
    try {
        const response = await fetch('/farms');
        if (response.ok) {
            const farms = await response.json();
            renderFarms(farms);
        } else {
            console.error('Failed to load farms:', await response.text());
        }
    } catch (error) {
        console.error('Error loading farms:', error);
    }
}

function renderFarms(farms) {
    const farmsDiv = document.getElementById('farms');
    farmsDiv.innerHTML = ''; // Clear the div before adding farms

    farms.forEach(farm => {
        const farmDiv = document.createElement('div');
        farmDiv.className = 'farm-item';
        farmDiv.innerHTML = `
            <p>Farm: ${farm.name} | Location: ${farm.location}</p>
            <button onclick="editFarm(${farm.id}, '${farm.name}', '${farm.location}')">Edit Farm</button>
            <button onclick="deleteFarm(${farm.id})">Delete Farm</button>
        `;

        if (farm.rigs) {
            farm.rigs.forEach(rig => {
                const rigDiv = document.createElement('div');
                rigDiv.className = 'rig-item';
                rigDiv.innerHTML = `
                    <p>Rig: ${rig.name} | Location: ${rig.location}</p>
                    <button onclick="moveRig(${rig.id})">Move Rig</button>
                `;
                rig.gpus.forEach(gpu => {
                    const gpuDiv = document.createElement('div');
                    gpuDiv.className = 'gpu-item';
                    gpuDiv.innerHTML = `GPU: ${gpu.name} | Temp: ${gpu.temp}°C | Watt: ${gpu.watt}W`;
                    rigDiv.appendChild(gpuDiv);
                });
                farmDiv.appendChild(rigDiv);
            });
        }

        farmsDiv.appendChild(farmDiv);
    });
}

async function createFarm() {
    const farmName = prompt('Enter Farm name:');
    const farmLocation = prompt('Enter Farm location:');

    if (farmName && farmLocation) {
        try {
            const response = await fetch('/farms', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name: farmName, location: farmLocation })
            });

            if (response.ok) {
                loadFarms(); // Refresh the list after creating the farm
            } else {
                console.error('Failed to create farm:', await response.text());
            }
        } catch (error) {
            console.error('Error creating farm:', error);
        }
    }
}

async function editFarm(farmId, oldName, oldLocation) {
    const newName = prompt('Enter new Farm name:', oldName);
    const newLocation = prompt('Enter new Farm location:', oldLocation);

    if (newName && newLocation) {
        try {
            const response = await fetch(`/farms/${farmId}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name: newName, location: newLocation })
            });

            if (response.ok) {
                loadFarms(); // Refresh the list after editing the farm
            } else {
                console.error('Failed to edit farm:', await response.text());
            }
        } catch (error) {
            console.error('Error editing farm:', error);
        }
    }
}

async function deleteFarm(farmId) {
    if (confirm('Are you sure you want to delete this farm?')) {
        try {
            const response = await fetch(`/farms/${farmId}`, { method: 'DELETE' });
            if (response.ok) {
                loadFarms(); // Refresh the list after deleting the farm
            } else {
                console.error('Failed to delete farm:', await response.text());
            }
        } catch (error) {
            console.error('Error deleting farm:', error);
        }
    }
}

function moveRig(rigId) {
    fetch('/farms')
        .then(response => response.json())
        .then(farms => {
            // Create the modal structure
            const modal = document.createElement('div');
            modal.classList.add('modal');
            modal.innerHTML = `
                <div class="modal-content">
                    <p>Select the farm to move this rig to:</p>
                    <select id="farmSelect">
                        ${farms.map(farm => `<option value="${farm.id}">${farm.name}</option>`).join('')}
                    </select>
                    <div class="modal-buttons">
                        <button id="cancelButton">Cancel</button>
                        <button id="okButton">OK</button>
                    </div>
                </div>
            `;
            document.body.appendChild(modal);
            console.log('Modal added to the DOM');

            // Handle OK button click
            const okButton = document.getElementById('okButton');
            if (okButton) {
                okButton.addEventListener('click', () => {
                    const selectedFarmId = parseInt(document.getElementById('farmSelect').value, 10); // Convert to integer
                    if (selectedFarmId) {
                        fetch(`/rigs/${rigId}/move`, {
                            method: 'PUT',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ farm_id: selectedFarmId }) // Send as integer
                        }).then(response => {
                            if (response.ok) {
                                loadFarms(); // Refresh the list after moving the rig
                            } else {
                                response.text().then(errorText => console.error('Failed to move rig:', errorText));
                            }
                            document.body.removeChild(modal); // Remove modal after action
                        }).catch(error => console.error('Error moving rig:', error));
                    }
                });
                console.log('OK button event listener added');
            }

            // Handle Cancel button click
            const cancelButton = document.getElementById('cancelButton');
            if (cancelButton) {
                cancelButton.addEventListener('click', () => {
                    document.body.removeChild(modal); // Remove modal without any action
                });
                console.log('Cancel button event listener added');
            }
        })
        .catch(error => console.error('Error loading farms:', error));
}

// Attach event listener to the "Create New Farm" button
document.getElementById('createFarm').addEventListener('click', createFarm);

// Load farms when the page is ready
document.addEventListener('DOMContentLoaded', loadFarms);