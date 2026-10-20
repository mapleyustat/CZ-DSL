document.addEventListener('DOMContentLoaded', () => {
    // 1. Activity Bar Navigation
    const activityBtns = document.querySelectorAll('.activity-btn[data-target]');
    activityBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            // Remove active class from all buttons
            activityBtns.forEach(b => b.classList.remove('active'));
            // Add active class to clicked button
            btn.classList.add('active');
            
            // In a real app, this would switch sidebar content
            const target = btn.dataset.target;
            console.log(`Switched to ${target}`);
        });
    });

    // 2. File Tree Expansion
    const folders = document.querySelectorAll('.tree-item.folder .tree-content');
    folders.forEach(folder => {
        folder.addEventListener('click', (e) => {
            const parent = folder.parentElement;
            parent.classList.toggle('expanded');
            parent.classList.toggle('collapsed');
            
            // Toggle icon
            const icon = folder.querySelector('.fa-chevron-down, .fa-chevron-right');
            if (icon) {
                if (parent.classList.contains('expanded')) {
                    icon.classList.replace('fa-chevron-right', 'fa-chevron-down');
                } else {
                    icon.classList.replace('fa-chevron-down', 'fa-chevron-right');
                }
            }
        });
    });

    // 3. Tab Management
    const tabs = document.querySelectorAll('.tab');
    tabs.forEach(tab => {
        tab.addEventListener('click', (e) => {
            if (e.target.closest('.tab-close')) return;
            
            tabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            
            // In a real app, switch editor content
            console.log(`Switched to tab: ${tab.querySelector('.tab-title').textContent}`);
        });
        
        const closeBtn = tab.querySelector('.tab-close');
        if (closeBtn) {
            closeBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                tab.remove();
            });
        }
    });

    // 4. Panel Tabs
    const panelTabs = document.querySelectorAll('.panel-tab');
    panelTabs.forEach(tab => {
        tab.addEventListener('click', () => {
            panelTabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            // Switch panel content logic here
        });
    });

    // 5. AI Assistant Chat
    const chatInput = document.querySelector('.chat-input input');
    const sendBtn = document.querySelector('.chat-input button');
    const messagesContainer = document.querySelector('.chat-messages');

    function addMessage(text, type) {
        const msgDiv = document.createElement('div');
        msgDiv.className = `message ${type}`;
        msgDiv.textContent = text;
        messagesContainer.appendChild(msgDiv);
        messagesContainer.scrollTop = messagesContainer.scrollHeight;
    }

    function handleSend() {
        const text = chatInput.value.trim();
        if (!text) return;
        
        addMessage(text, 'user');
        chatInput.value = '';
        
        // Simulate AI response
        setTimeout(() => {
            addMessage("I'm analyzing your request... (This is a demo response)", 'ai');
        }, 1000);
    }

    sendBtn.addEventListener('click', handleSend);
    chatInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleSend();
    });
});
