<script lang="ts" setup>

const { pending, error, data: posts } = await useFetch("http://localhost:8081/api/posts")

</script>

<template>
	<div>
		<p v-if="pending">Loading</p>
		<p v-if="error">Error connecting to backend!</p>
		<div class="posts-container" v-else v-for="post in posts.posts">
			<PostCard :post="post"/>
		</div>


	</div>
</template>

<script lang="ts">
import PostCard from "~/components/PostCard.vue";

export default defineComponent({
  components: {
    PostCard
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
