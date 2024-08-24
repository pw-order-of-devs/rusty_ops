<div class="loader-layer">
	<div class="loader" />
</div>

<style lang="scss">
	@import 'src/styles/global';

	.loader-layer {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 9999;
		background-color: rgba($color-primary-1, 0.75);

		.loader {
			width: 10rem;
			aspect-ratio: 1;
			border-radius: 10rem;
			background:
				radial-gradient(farthest-side, $color-secondary-2 94%, rgba($color-primary-1, 0)) top/0.4rem
					0.4rem no-repeat,
				conic-gradient(rgba($color-primary-1, 0) 30%, $color-secondary-2);
			mask: radial-gradient(
				farthest-side,
				rgba($color-primary-1, 0) calc(100% - 0.7rem),
				$color-primary-1 0
			);
			animation: l13 1s infinite linear;
		}

		@keyframes l13 {
			100% {
				transform: rotate(1turn);
			}
		}
	}
</style>
