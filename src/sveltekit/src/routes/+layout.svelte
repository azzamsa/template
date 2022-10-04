<script lang="ts">
 import NavBar from '$lib/navbar/NavBar.svelte'
 import '../app.postcss'
</script>

<div class="pt-4 mx-4">
    <NavBar />
</div>

<main>
    <slot />
</main>
