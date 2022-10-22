<script>
	export let opend = false
</script>
<style>
	svg {
		min-height: 24px;
		transition: transform 0.3s ease-in-out;
	}
	
	svg line {
		stroke: currentColor;
		stroke-width: 3;
		transition: transform 0.3s ease-in-out
	}
	
	button {
		z-index: 20;
	}
	
	.opend svg {
		transform: scale(0.7)
	}
	
	.opend #top {
		transform: translate(6px, 0px) rotate(45deg)
	}
	
	.opend #middle {
		opacity: 0;
	}
	
  .opend #bottom {
		transform: translate(-12px, 9px) rotate(-45deg)
	}
</style>