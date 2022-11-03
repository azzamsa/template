<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
  import Counter from "./lib/Counter.svelte";
</script>

<main class="flex flex-col text-center text-black">
  <div class="mt-60 flex justify-center space-x-10">
    <a href="https://vitejs.dev" target="_blank" rel="noopener noreferrer">
      <img class="h-24" src="/vite.svg" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noopener noreferrer">
      <img class="h-24" src={svelteLogo} alt="Svelte Logo" />
    </a>
  </div>

  <h1 class="mt-20 text-5xl font-bold">
    <span class="text-purple-500">Vite</span>
    <span>+</span>
    <span class="text-red-600">Svelte</span>
  </h1>

  <div class="mt-12">
    <Counter />
  </div>

  <p class="mt-12">
    Check out <a
      href="https://github.com/sveltejs/kit#readme"
      target="_blank"
      rel="noopener noreferrer">SvelteKit</a
    >, the official Svelte app framework powered by Vite!
  </p>

  <p class="mt-5 text-gray-500">Click on the Vite and Svelte logos to learn more</p>
</main>

<style lang="postcss">
  main {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  }

  a {
    @apply text-blue-500;
  }
</style>
