document.addEventListener('DOMContentLoaded', function() {
    loadFarms();

    const overviewTab = document.getElementById("overviewTab");
    const systemOptionsTab = document.getElementById("systemOptionsTab");
    const overviewContainer = document.getElementById("overview-container");
    const systemOptionsContainer = document.getElementById("system-options-container");

    const logoContainer = document.querySelector(".container_logo");
    const farmModal = document.getElementById("farmModal");
    const farmSelect = document.getElementById("farmSelect");
    const rigsContainer = document.getElementById("rigsContainer");
    const rigsList = document.getElementById("rigsList");

    // Tab switching logic
    overviewTab.addEventListener("click", function() {
        overviewTab.classList.add("active");
        systemOptionsTab.classList.remove("active");
        overviewContainer.classList.remove("hidden");
        systemOptionsContainer.classList.add("hidden");
    });

    systemOptionsTab.addEventListener("click", function() {
        systemOptionsTab.classList.add("active");
        overviewTab.classList.remove("active");
        systemOptionsContainer.classList.remove("hidden");
        overviewContainer.classList.add("hidden");
    });

    // Open modal
    logoContainer.addEventListener("click", function() {
        farmModal.style.display = "block";
        loadFarmsIntoSelect();
    });

    // Close modal
    document.getElementById("closeModal").addEventListener("click", function() {
        farmModal.style.display = "none";
    });

    // Edit, Delete, Move Rig button logic
    document.getElementById("editFarm").addEventListener("click", function() {
        const selectedFarmId = farmSelect.value;
        const selectedFarm = farmSelect.options[farmSelect.selectedIndex].text;
        const newName = prompt('Enter new Farm name:', selectedFarm);
        if (newName) {
            editFarm(selectedFarmId, newName);
        }
    });

    document.getElementById("deleteFarm").addEventListener("click", function() {
        const selectedFarmId = farmSelect.value;
        deleteFarm(selectedFarmId);
    });

    document.getElementById("moveRig").addEventListener("click", function() {
        rigsContainer.classList.remove("hidden");
        loadRigsForFarm(farmSelect.value);
    });

    document.getElementById("confirmMoveRigs").addEventListener("click", function() {
        const selectedRigs = Array.from(rigsList.querySelectorAll('input:checked')).map(input => input.value);
        moveRigsToFarm(selectedRigs, farmSelect.value);
    });

    document.getElementById("showRigs").addEventListener("click", function() {
        const selectedFarmId = farmSelect.value;
        showFarmWithRigs(selectedFarmId);
    });
});

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

async function loadFarmsIntoSelect() {
    try {
        const response = await fetch('/farms');
        if (response.ok) {
            const farms = await response.json();
            farmSelect.innerHTML = farms.map(farm => `<option value="${farm.id}">${farm.name}</option>`).join('');
        } else {
            console.error('Failed to load farms into select:', await response.text());
        }
    } catch (error) {
        console.error('Error loading farms into select:', error);
    }
}

function renderFarms(farms) {
    const farmsDiv = document.getElementById('farms');
    farmsDiv.innerHTML = ''; // Clear the div before adding farms

    farms.forEach(farm => {
        const farmDiv = document.createElement('div');
        farmDiv.className = 'farm-item';
        farmDiv.innerHTML = `
            <p>Farm: ${farm.name} | Location: ${farm.location || 'Unknown'}</p>
            <button onclick="editFarm(${farm.id}, '${farm.name}', '${farm.location}')">Edit Farm</button>
            <button onclick="deleteFarm(${farm.id})">Delete Farm</button>
        `;
        farmsDiv.appendChild(farmDiv);
    });
}

async function editFarm(farmId, newName) {
    try {
        const response = await fetch(`/farms/${farmId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: newName })
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

async function loadRigsForFarm(farmId) {
    try {
        const response = await fetch(`/farms/${farmId}/rigs`); // Ensure this endpoint exists
        if (response.ok) {
            const rigs = await response.json();
            const rigsList = document.getElementById('rigsList');
            rigsList.innerHTML = rigs.map(rig => `
                <div>
                    <input type="checkbox" value="${rig.id}"> ${rig.name}
                </div>
            `).join('');
        } else {
            console.error('Failed to load rigs:', await response.text());
        }
    } catch (error) {
        console.error('Error loading rigs:', error);
    }
}

async function showFarmWithRigs(farmId) {
    try {
        const response = await fetch(`/farms/${farmId}`);
        if (response.ok) {
            const farm = await response.json();
            const farmsDiv = document.getElementById('farms');
            farmsDiv.innerHTML = ''; // Clear the div before showing the farm

            const farmDiv = document.createElement('div');
            farmDiv.className = 'farm-item';
            farmDiv.innerHTML = `
                <p>Farm: ${farm.name} | Location: ${farm.location || 'Unknown'}</p>
            `;

            farm.rigs.forEach(rig => {
                const rigDiv = document.createElement('div');
                rigDiv.className = 'rig-item';
                rigDiv.innerHTML = `<p>Rig: ${rig.name} | Location: ${rig.location}</p>`;

                rig.gpus.forEach(gpu => {
                    const gpuDiv = document.createElement('div');
                    gpuDiv.className = 'gpu-item';
                    gpuDiv.innerHTML = `GPU: ${gpu.name} | Temp: ${gpu.temp}°C | Watt: ${gpu.watt}W`;
                    rigDiv.appendChild(gpuDiv);
                });

                farmDiv.appendChild(rigDiv);
            });

            farmsDiv.appendChild(farmDiv);
        } else {
            console.error('Failed to show farm:', await response.text());
        }
    } catch (error) {
        console.error('Error showing farm:', error);
    }
}

async function moveRigsToFarm(rigIds, farmId) {
    try {
        const response = await fetch(`/rigs/move`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ rigIds, farmId })
        });

        if (response.ok) {
            loadFarms(); // Refresh the list after moving rigs
            rigsContainer.classList.add("hidden");
        } else {
            console.error('Failed to move rigs:', await response.text());
        }
    } catch (error) {
        console.error('Error moving rigs:', error);
    }
}

// Attach event listener to the "Create New Farm" button
document.getElementById('createFarm').addEventListener('click', async function() {
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
});

// Additional event listeners for the system control buttons (as before)
document.getElementById('startErgo').addEventListener('click', () => {
    fetch('/send-command/start_ergo', { method: 'POST' });
});
document.getElementById('startXel').addEventListener('click', () => {
    fetch('/send-command/start_xel', { method: 'POST' });
});
document.getElementById('startRVN').addEventListener('click', () => {
    fetch('/send-command/start_rvn', { method: 'POST' });
});
document.getElementById('startFish').addEventListener('click', () => {
    fetch('/send-command/start_fish', { method: 'POST' });
});
document.getElementById('startFlux').addEventListener('click', () => {
    fetch('/send-command/start_flux', { method: 'POST' });
});
document.getElementById('startBeam').addEventListener('click', () => {
    fetch('/send-command/start_beam', { method: 'POST' });
});
document.getElementById('stopMining').addEventListener('click', () => {
    fetch('/send-command/stop_mining', { method: 'POST' });
});
document.getElementById('adjustOverclock').addEventListener('click', () => {
    fetch('/send-command/adjust_overclock', { method: 'POST' });
});
document.getElementById('rebootGPU').addEventListener('click', () => {
    fetch('/send-command/reboot_gpu', { method: 'POST' });
});
document.getElementById('rebootRig').addEventListener('click', () => {
    fetch('/send-command/reboot_rig', { method: 'POST' });
});
document.getElementById('rebootAllRigs').addEventListener('click', () => {
    fetch('/send-command/reboot_all_rigs', { method: 'POST' });
});
document.getElementById('updateSoftware').addEventListener('click', () => {
    fetch('/send-command/update_software', { method: 'POST' });
});