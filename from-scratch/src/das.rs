use kzg::Fr;
use crate::kzg_types::FFTSettings;
use blst::{blst_fr_add, blst_fr_sub, blst_fr_mul};

pub fn das_fft_extension(ab: &mut [Fr], stride: usize, fft_settings: &FFTSettings) {
    if ab.len() < 2 {
        return;
    } else if ab.len() == 2 {
        let mut x: Fr = Fr::default();
        let mut y: Fr = Fr::default();
        let mut temp: Fr = Fr::default();

        unsafe {
            blst_fr_add(&mut x, &ab[0], &ab[1]);
            blst_fr_sub(&mut y, &ab[0], &ab[1]);
            blst_fr_mul(&mut temp, &y, &fft_settings.expanded_roots_of_unity[stride]);
            blst_fr_add(&mut ab[0], &x, &temp);
            blst_fr_sub(&mut ab[1], &x, &temp);
        }
    } else {
        let half: usize = ab.len();
        let halfhalf: usize = half / 2;

        for i in 0..halfhalf {
            let mut tmp1: Fr = Fr::default();
            let mut tmp2: Fr = Fr::default();

            unsafe {
                blst_fr_add(&mut tmp1, &ab[i], &ab[halfhalf + i]);
                blst_fr_sub(&mut tmp2, &ab[i], &ab[halfhalf + i]);
                blst_fr_mul(&mut ab[halfhalf + i], &tmp2, &fft_settings.reverse_roots_of_unity[i * 2 * stride]);

                ab[i] = tmp1;
            }
        }

        // Recurse
        das_fft_extension(&mut ab[..halfhalf], stride * 2, &fft_settings);
        das_fft_extension(&mut ab[halfhalf..], stride * 2, &fft_settings);

        for i in 0..halfhalf {
            let mut y_times_root: Fr = Fr::default();
            let x = ab[i];
            let y = ab[halfhalf + i];

            unsafe {
                blst_fr_mul(&mut y_times_root, &y, &fft_settings.expanded_roots_of_unity[(1 + 2 * i) * stride]);
                blst_fr_add(&mut ab[i], &x, &y_times_root);
                blst_fr_sub(&mut ab[halfhalf + i], &x, &y_times_root);
            }
        }
    }
}