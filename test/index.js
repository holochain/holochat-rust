// This test file uses the tape testing framework. 
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")
const app2 = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

const testNewChannelParams = {
  name: "test new channel",
  public: true
}

const testMessage = {
  timestamp: "100000",
  text : "Some text"
}

test('Can create a public channel with no other members and retrieve it', (t) => {
  const create_result = app.call('chat', 'main', 'create_channel', testNewChannelParams)
  console.log(create_result)
  t.deepEqual(create_result.address.length, 46)

  const get_result = app.call('chat', 'main', 'get_my_channels', {})
  console.log(get_result)
  t.deepEqual(get_result.length, 1)

  t.end()
})

test('Can post a message to the channel and retrieve', (t) => {
  const create_result = app.call('chat', 'main', 'create_channel', testNewChannelParams)
  console.log(create_result)
  const channel_addr = create_result.address
  t.deepEqual(channel_addr.length, 46)

  const get_result = app.call('chat', 'main', 'get_my_channels', {})
  console.log(get_result)
  t.deepEqual(get_result.length, 1)

  const post_result = app.call('chat', 'main', 'post_message', {channel_name: testNewChannelParams.name, message: testMessage})
  console.log(post_result)
  t.deepEqual(post_result, {Ok: { success: true}})

  const get_message_result = app.call('chat', 'main', 'get_messages', {channel_name: testNewChannelParams.name, min_count: 10})
  console.log(get_message_result)
  t.deepEqual(get_message_result[0], testMessage)
  t.end()
})

test('scenario test create & publish post -> get from other instance', (t) => {
  t.plan(3)

  const create_result = app.call("chat", "main", "create_channel", testNewChannelParams)

  t.equal(create_result.address.length, 46)
  t.equal(create_result.address, "QmSZFNPn4zgtgS18J2XZcXmvw7Rv1W1Nmf8QH3aVeJjUMY")

  const check_get_result = function check_get_result (i = 0, get_result) {
    t.comment('checking get result for the ' + i + 'th time')
    t.comment(get_result + "")

    if (get_result) {
      t.equal(get_result[0].name, testNewChannelParams.name)
    }
    else if (i < 50) {
      setTimeout(function() {
        check_get_result(
          ++i,
          app2.call("chat", "main", "get_my_channels", {})
        )
      }, 100)
    }
    else {
      t.end()
    }

  }()
})


