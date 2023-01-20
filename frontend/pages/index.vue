<template>
	<div>
		<p v-if="$fetchState.pending">Loading</p>
		<p v-if="$fetchState.error">Error connecting to backend!</p>
		<div class="posts-container" v-else v-for="post in posts_data.posts">
			<PostCard :post="post"/>
		</div>


	</div>
</template>

<script lang="ts">
	import Vue from "vue";

	export default Vue.extend({
		data() {
			return {
				posts_data: {}
			}
		},
		async fetch() {
			this.posts_data = await fetch(
				"http://localhost:8081/api/posts"
			).then(res => res.json())
		}
	})
</script>

<style lang="scss" scoped>
.posts-container {
	*, :not(&:last-child) {
		margin-bottom: 1rem;
	}
}
</style>
