use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_async_ecs::*;
use bevy_ecs::world::CommandQueue;
use bevy_log::LogPlugin;

// vanilla Bevy system
fn print_names(query: Query<(Entity, &Name)>) {
	for (id, name) in query.iter() {
		info!("entity {:?} has name '{}'", id, name);
	}
}

fn main() {
	App::new()
		.add_plugins((MinimalPlugins, LogPlugin::default(), AsyncEcsPlugin))
		.add_systems(Startup, |world: &mut World| {

			let async_world = AsyncWorld::from_world(world);
			//info!("closed: {}", async_world.0.0.is_closed());
	
			let fut = async move {
				//info!("future started");
				let print_names = async_world.register_system(print_names).await;
			
				//info!("closed: {}", async_world.0.0.is_closed());
				//TimeoutFuture::new(1_000).await;
				let entity = async_world.spawn_named("Frank").await;
				//info!("closed: {}", async_world.0.0.is_closed());
				print_names.run().await;
				// entity.despawn().await;
				//info!("done! you can close the window");
			};

			AsyncComputeTaskPool::get().spawn(fut).detach();
		})
		.run();
}
