
// 1. In production code, it is recommended to use AutoSafe instead of AlwaysSafe, 
// this will eliminate unnecessary checks in the release build, but leave 
// them in the test build.
//
// 2. It is generally recommended to use Panic or Abort as a trigger for undefined behavior.
//
use SafeManuallyDrop::AlwaysSafePanicManuallyDrop as ManuallyDrop;

#[derive(Default, Debug)]
struct ControlDrop(usize);

// Properly created and validated MyLogicData structure.
#[derive(Default)]
struct MyLogicData {
	data: ManuallyDrop<ControlDrop>
}

impl MyLogicData {
	/// Exceptional logic. As a result, the original value will always be returned.
	pub fn ignore_mylogic_and_getdata(mut self) -> ControlDrop {
		// Note that you can only `take` once, any further operation with 
		// ManuallyDrop will cause a panic.
		let data = unsafe {
			ManuallyDrop::take(&mut self.data)
		};
		
		// ManuallyDrop::forget analog forget(self).
		ManuallyDrop::forget(self);
		
		/*
			data logic
		*/
		
		data
	}
}

impl Drop for MyLogicData {
	fn drop(&mut self) {
		/*
			def logic
		*/
		println!("MyLogicData, indata: {:?}", self.data);
		
		/*
			Notification
			1. `ManuallyDrop` always requires it to be freed when it is no longer needed.
			2. Once `ManuallyDrop` is freed, you will not be able to read data from it
			3. You cannot drop `ManuallyDrop` twice.
			...
			
			You can remove the `unsafe` flags if you don't use the `always_compatible_stdapi` flag.
		*/
		unsafe {
			ManuallyDrop::drop(&mut self.data);
		}
	}
}

fn main() {
	{
		// run my logic
		let indata = MyLogicData::default();
		drop(indata);
		
		// This case will just make the logic default by executing the code in drop.
	}
	{
		// ignore_mylogic
		let indata = MyLogicData::default();
		let cd_data = indata.ignore_mylogic_and_getdata();
	
		println!("ignore_mylogic: {:?}", cd_data);
		
		// In this case, the standard reset logic is eliminated and another 
		// specific principle is used, which is embedded in the function with data return.
	}
}
