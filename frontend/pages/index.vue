<template>
	<div>
		<p v-if="$fetchState.pending">Loading</p>
		<p v-if="$fetchState.error">Error occured</p>
		<div v-else v-for="post in posts_data.posts" :key="post">
			<h1>
				{{ post.title }}
			</h1>
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
