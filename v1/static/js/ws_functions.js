// ws_functions.js
document.addEventListener('DOMContentLoaded', function() {
    const socket = new WebSocket("ws://127.0.0.1:8080/ws/frontend");

    socket.addEventListener('open', function (event) {
        console.log("WebSocket is connected.");
    });

    socket.addEventListener('message', function (event) {
        const message = JSON.parse(event.data);
        console.log('Message from server: ', message);

        if (message.type === "SHOW_RIGS_RESPONSE") {
            // Handle the response when showing rigs
            const farmData = JSON.parse(message.data);
            populateFarmDropdown(farmData.farms);
        }
    });

    const systemButtons = {
        'createFarmInModal': function() {
            const farmName = prompt('Enter Farm name:');
            const farmLocation = prompt('Enter Farm location:');
            if (farmName && farmLocation) {
                socket.send(JSON.stringify({ type: "CREATE_FARM", name: farmName, location: farmLocation }));
            }
        },
        'editFarm': function() {
            const selectedFarmId = farmSelect.value;
            const newName = prompt('Enter new Farm name:', farmSelect.options[farmSelect.selectedIndex].text);
            if (newName) {
                socket.send(JSON.stringify({ type: "EDIT_FARM", id: selectedFarmId, name: newName }));
            }
        },
        'deleteFarm': function() {
            const selectedFarmId = farmSelect.value;
            if (confirm('Are you sure you want to delete this farm?')) {
                socket.send(JSON.stringify({ type: "DELETE_FARM", id: selectedFarmId }));
            }
        },
        'showRigs': function() {
            const selectedFarmId = farmSelect.value;
            socket.send(JSON.stringify({ type: "SHOW_RIGS", id: selectedFarmId }));
        },
        'startErgo': 'start_ergo',
        'startXel': 'start_xel',
        'startRVN': 'start_rvn',
        'startFish': 'start_fish',
        'startFlux': 'start_flux',
        'startBeam': 'start_beam',
        'stopMining': 'stop_mining',
        'adjustOverclock': 'adjust_overclock',
        'rebootGPU': 'reboot_gpu',
        'rebootRig': 'reboot_rig',
        'rebootAllRigs': 'reboot_all_rigs',
        'updateSoftware': 'update_software'
    };

    for (const [buttonId, commandOrFunction] of Object.entries(systemButtons)) {
        document.getElementById(buttonId).addEventListener('click', () => {
            if (typeof commandOrFunction === 'function') {
                commandOrFunction();
            } else {
                console.log(`Sending command: ${commandOrFunction}`);
                socket.send(commandOrFunction);
            }
        });
    }

    function populateFarmDropdown(farms) {
        const farmSelect = document.getElementById('farmSelect');
        farmSelect.innerHTML = ''; // Clear existing options

        farms.forEach(farm => {
            const option = document.createElement('option');
            option.value = farm.id;
            option.text = farm.name;
            farmSelect.appendChild(option);
        });
    }
});