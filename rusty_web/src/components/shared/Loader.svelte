<div class="loader-layer">
	<div class="loader" />
</div>

<style lang="scss">
	.loader-layer {
		width: 100vw;
		height: 100vh;
		position: absolute;
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 9999;
		background-color: #1b1b1bbb;
		.loader {
			width: 10rem;
			aspect-ratio: 1;
			border-radius: 10rem;
			background:
				radial-gradient(farthest-side, #e2e2e2 94%, #0000) top/8px 8px no-repeat,
				conic-gradient(#0000 30%, #e2e2e2);
			mask: radial-gradient(farthest-side, #0000 calc(100% - 0.7rem), #000 0);
			animation: l13 1s infinite linear;
		}
		@keyframes l13 {
			100% {
				transform: rotate(1turn);
			}
		}
	}
</style>
