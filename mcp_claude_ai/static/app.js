// app.js
document.addEventListener('DOMContentLoaded', () => {
    // DOM elements
    const searchForm = document.getElementById('search-form');
    const searchQuery = document.getElementById('search-query');
    const searchResults = document.getElementById('search-results');
    
    const generateForm = document.getElementById('generate-form');
    const promptInput = document.getElementById('prompt');
    const temperatureSlider = document.getElementById('temperature');
    const tempValue = document.getElementById('temp-value');
    const maxTokensSlider = document.getElementById('max-tokens');
    const tokensValue = document.getElementById('tokens-value');
    const contextList = document.getElementById('context-list');
    const generateBtn = document.getElementById('generate-btn');
    const loadingElem = document.getElementById('loading');
    const resultElem = document.getElementById('result');
    const completionElem = document.getElementById('completion');
    const usageElem = document.getElementById('usage');
    
    const addContextForm = document.getElementById('add-context-form');
    const contextIdInput = document.getElementById('context-id');
    const contextContentInput = document.getElementById('context-content');
    const contextMetadataInput = document.getElementById('context-metadata');
    
    // API URL (updated to port 3030)
    const API_URL = 'http://localhost:3030/api';
    
    // Selected context items for generation
    const selectedContexts = [];
    
    // Update slider values
    temperatureSlider.addEventListener('input', () => {
        tempValue.textContent = temperatureSlider.value;
    });
    
    maxTokensSlider.addEventListener('input', () => {
        tokensValue.textContent = maxTokensSlider.value;
    });
    
    // Search for contexts
    searchForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const query = searchQuery.value.trim();
        
        if (!query) return;
        
        try {
            searchResults.innerHTML = 'Searching...';
            
            const response = await fetch(`${API_URL}/search?query=${encodeURIComponent(query)}`);
            
            if (!response.ok) {
                throw new Error('Search failed');
            }
            
            const results = await response.json();
            
            if (results.length === 0) {
                searchResults.innerHTML = '<p>No results found.</p>';
                return;
            }
            
            renderSearchResults(results);
        } catch (error) {
            console.error('Error searching contexts:', error);
            searchResults.innerHTML = '<p>Error searching contexts. Please try again.</p>';
        }
    });
    
    // Generate text with Claude
    generateForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const prompt = promptInput.value.trim();
        if (!prompt) return;
        
        try {
            // Show loading state
            loadingElem.classList.remove('hidden');
            resultElem.classList.add('hidden');
            generateBtn.disabled = true;
            
            const requestData = {
                prompt,
                max_tokens: parseInt(maxTokensSlider.value),
                temperature: parseFloat(temperatureSlider.value),
                context: selectedContexts.length > 0 ? selectedContexts : undefined
            };
            
            const response = await fetch(`${API_URL}/generate`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(requestData)
            });
            
            if (!response.ok) {
                throw new Error('Generation failed');
            }
            
            const result = await response.json();
            
            // Display the result
            completionElem.textContent = result.completion;
            usageElem.textContent = `Model: ${result.model} | Tokens: ${result.usage.total_tokens} (${result.usage.prompt_tokens} prompt, ${result.usage.completion_tokens} completion)`;
            
            resultElem.classList.remove('hidden');
        } catch (error) {
            console.error('Error generating with Claude:', error);
            completionElem.textContent = 'Error generating text. Please try again.';
            usageElem.textContent = '';
            resultElem.classList.remove('hidden');
        } finally {
            loadingElem.classList.add('hidden');
            generateBtn.disabled = false;
        }
    });
    
    // Add new context
    addContextForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const id = contextIdInput.value.trim();
        const content = contextContentInput.value.trim();
        if (!content) return;
        
        let metadata = null;
        try {
            const metadataText = contextMetadataInput.value.trim();
            if (metadataText) {
                metadata = JSON.parse(metadataText);
            }
        } catch (error) {
            alert('Invalid JSON in metadata field');
            return;
        }
        
        try {
            const response = await fetch(`${API_URL}/contexts`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    id,
                    content,
                    metadata
                })
            });
            
            if (!response.ok) {
                throw new Error('Failed to add context');
            }
            
            const newContext = await response.json();
            alert('Context added successfully!');
            
            // Clear the form
            contextIdInput.value = '';
            contextContentInput.value = '';
            contextMetadataInput.value = '';
            
        } catch (error) {
            console.error('Error adding context:', error);
            alert('Error adding context. Please try again.');
        }
    });
    
    // Render search results
    function renderSearchResults(results) {
        searchResults.innerHTML = '';
        
        results.forEach(context => {
            const resultItem = document.createElement('div');
            resultItem.className = 'result-item';
            
            const contentPreview = context.content.length > 200 
                ? context.content.substring(0, 200) + '...' 
                : context.content;
            
            resultItem.innerHTML = `
                <h3>${context.id || 'Unnamed Context'}</h3>
                <div class="result-item-content">${escapeHtml(contentPreview)}</div>
                <div class="result-item-controls">
                    <button class="use-context">Use in Generation</button>
                </div>
            `;
            
            // Add event listener to the "Use in Generation" button
            const useButton = resultItem.querySelector('.use-context');
            useButton.addEventListener('click', () => {
                addToSelectedContexts(context);
            });
            
            searchResults.appendChild(resultItem);
        });
    }
    
    // Add a context to the selected contexts for generation
    function addToSelectedContexts(context) {
        // Check if already added
        if (selectedContexts.some(c => c.id === context.id)) {
            alert('This context is already selected');
            return;
        }
        
        // Add to the selected contexts array
        selectedContexts.push({
            id: context.id,
            content: context.content,
            metadata: context.metadata
        });
        
        // Update the UI
        updateSelectedContextsList();
    }
    
    // Update the selected contexts list UI
    function updateSelectedContextsList() {
        contextList.innerHTML = '';
        
        if (selectedContexts.length === 0) {
            contextList.innerHTML = '<li>No contexts selected</li>';
            return;
        }
        
        selectedContexts.forEach((context, index) => {
            const listItem = document.createElement('li');
            const contextName = context.id || `Context ${index + 1}`;
            
            listItem.innerHTML = `
                <span>${escapeHtml(contextName)}</span>
                <button class="remove-context" data-index="${index}">Remove</button>
            `;
            
            const removeButton = listItem.querySelector('.remove-context');
            removeButton.addEventListener('click', () => {
                selectedContexts.splice(index, 1);
                updateSelectedContextsList();
            });
            
            contextList.appendChild(listItem);
        });
    }
    
    // Helper function to escape HTML special characters
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    // Initialize the UI
    updateSelectedContextsList();
    
    // Fetch and display all contexts on page load (optional)
    async function fetchAllContexts() {
        try {
            const response = await fetch(`${API_URL}/contexts`);
            
            if (!response.ok) {
                console.error('Failed to fetch contexts');
                return;
            }
            
            const contexts = await response.json();
            
            if (contexts.length > 0) {
                renderSearchResults(contexts);
            }
        } catch (error) {
            console.error('Error fetching contexts:', error);
        }
    }
    
    // Uncomment the line below to fetch all contexts on page load
    // fetchAllContexts();
});
