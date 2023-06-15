<script lang="ts">
  async function fetchData() {
    const res = await fetch("/api/users");
    const data = await res.json();

    console.log(data);

    if (res.ok) {
      return data;
    } else {
      throw new Error(data);
    }
  }

  let userPromise = fetchData();

  const buttonHandler = () => (userPromise = fetchData());
</script>

<button on:click={buttonHandler}>Reload</button>

{#await userPromise}
  <p>loading</p>
{:then users}
  <ul>
    {#each users as user}
      <li>[{user.id}] {user.username}</li>
    {/each}
  </ul>
{:catch error}
  <p style="color: red">{error.message}</p>
{/await}
