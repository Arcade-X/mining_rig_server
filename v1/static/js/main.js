document.addEventListener('DOMContentLoaded', function() {
    // Tab switching logic
    const systemOptionsTab = document.getElementById("systemOptionsTab");
    const overviewTab = document.getElementById("overviewTab");
    const systemOptionsContainer = document.getElementById("system-options-container");
    const overviewContainer = document.getElementById("overview-container");

    systemOptionsTab.addEventListener("click", function() {
        systemOptionsTab.classList.add("active");
        overviewTab.classList.remove("active");
        systemOptionsContainer.classList.remove("hidden");
        overviewContainer.classList.add("hidden");
    });

    overviewTab.addEventListener("click", function() {
        overviewTab.classList.add("active");
        systemOptionsTab.classList.remove("active");
        overviewContainer.classList.remove("hidden");
        systemOptionsContainer.classList.add("hidden");
    });

    // Logo click opens the modal
    const logoContainer = document.querySelector('.container_logo');
    const farmModal = document.getElementById('farmModal');
    const closeModalButton = document.getElementById('closeModal');
    const modalContent = document.querySelector('.modal-content');

    // Function to show the modal
    function showModal() {
        farmModal.style.display = 'block';
    }

    // Function to hide the modal
    function hideModal() {
        farmModal.style.display = 'none';
    }

    // Add event listener to logo container to open modal on click
    logoContainer.addEventListener('click', showModal);

    // Add event listener to close button inside the modal
    closeModalButton.addEventListener('click', hideModal);

    // Add event listener to close the modal when clicking outside of the modal content
    window.addEventListener('click', function(event) {
        // Check if the click happened outside of the modal content and not on the logo container
        if (!modalContent.contains(event.target) && event.target !== logoContainer && !logoContainer.contains(event.target)) {
            hideModal();
        }
    });
});