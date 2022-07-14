<script>
	import MiniNavbar from './MiniNavBar.svelte'
	import Sidebar from './Sidebar.svelte'

	let opend = false
</script>

<Sidebar bind:opend/>
<MiniNavbar bind:sidebar={opend}/>
<svelte:head>
	<link href="https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css" rel="stylesheet"/>
</svelte:head>

<style>
	:global(body) {
		padding: 0;
	}
</style>