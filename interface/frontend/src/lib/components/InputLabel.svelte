<script lang="ts">
    import { onMount } from 'svelte';
  
    let inputValue: string = "";
    let result: number | null = null;
  
    const calculate = () => {
      result = 999.9; // Replace this with your actual calculation logic
    };
  
    const handleKeyPress = (event: KeyboardEvent) => {
      if (event.key === "Enter") {
        calculate();
      }
    };
  
    onMount(() => {
      // Focus the input field when the component is mounted
      const inputElement = document.getElementById("inputField") as HTMLInputElement;
      inputElement.focus();
    });
  </script>
  
<div class="input-container">
  <input
    type="text"
    id="inputField"
    class="input-field"
    bind:value={inputValue}
    on:keydown={handleKeyPress}
    placeholder="your input"
  />
</div>

{#if result !== null}
  <p class="result">Result: {result}</p>
{/if}

<style>
    :root {
        --color: #012e0f;
    }

    .input-container {
      align-items: center;
      justify-content: center;
      margin: -10px;
      width: 100dvw;
    }
  
    .input-field {
      border: 4px solid var(--color);
      border-radius: 2px;
      background-color: white;
      color: var(--color);
      font-size: 28px;
      padding: 10px 15px;
      outline: none; 
      font-weight: bold; 
      width: 100%;
    }
  
    .input-field::placeholder {
      color: var(--color); 
      font-weight: bold; 
      opacity: 0.6; 
    }
  
    .result {
      margin-top: 10px;
      color: var(--color);
      font-size: 28px;
      font-weight: bold;
    }
  </style>